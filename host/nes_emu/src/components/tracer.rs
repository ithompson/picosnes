use core::fmt;
use std::{cell::RefCell, collections::HashMap, fs::File, io::BufWriter, io::Write};

/// Trait for values that can be traced. Used in the register
/// tracing system to format their output.
pub trait TraceableValue {
    fn fmt_trace(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

/// Wrapper struct to use the TraceableValue formatter on a value when displaying it
struct TraceDisplay<'a, T: TraceableValue + ?Sized>(&'a T);
impl<'a, T: TraceableValue + ?Sized> fmt::Display for TraceDisplay<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_trace(f)
    }
}

/// A single element in the trace hierarchy. The existence of a TraceElement
/// node indicates that a particular part of the hierarchy is being traced.
#[derive(Debug)]
struct TraceElement {
    /// The full hierarchical name of this element
    full_name: String,
    /// A table of all children of this element, keyed by their name
    children: HashMap<String, TraceElementId>,
    /// Whether all children of this element are also enabled for tracing
    all_children_enabled: bool,
}

/// Handle for a trace element
#[derive(Debug, Clone, Copy)]
pub struct TraceElementId(usize);

impl TraceElementId {
    /// Whether this element is enabled for emitting trace data
    pub fn enabled(&self) -> bool {
        self.0 != usize::MAX
    }
}
/// Handle for the invisible unnamed root element
const ROOT_ELEMENT_ID: TraceElementId = TraceElementId(0);
/// Handle for a disabled trace element. Disabled elements are ignored when
/// emitting trace data
const DISABLED_ELEMENT_ID: TraceElementId = TraceElementId(usize::MAX);

/// Inner data for the tracer. Separated out from the root tracer object
/// to allow for interior mutability, specifically to enable writing to
/// a file
#[derive(Debug)]
struct TraceData {
    /// The tree of all trace elements, represented as a table for
    /// easy lookup by ID. Index 0 is always the root
    elements: Vec<TraceElement>,
    /// Optional output file for trace data
    trace_writer: Option<BufWriter<File>>,
}

/// A tracer for detailed emulator events
///
/// The tracer is organized into a hierarchy of trace elements, meant to represent different
/// components on the emulator. For example, one trace element may be "cpu", which has a child
/// named "regs" for the CPU registers. "regs" then has children "a", "x", "y", "pc", etc. for
/// all of the traced registers.
///
/// When constructing the tracer, you specify the trace element hierarchies you want to enable
/// in a dot-separated format. For example, the hierarchy "cpu.regs" enables tracing for "cpu.regs"
/// trace events and all of its children, such as "cpu.regs.a" or "cpu.regs.pc".
///
/// Trace data is formatted as simple strings, with no structured meaning enforced by this tracer.
/// Each emulator component can define its trace data and hierarchy as it sees fit.
///
/// Basic example usage:
/// ```
/// let tracer = Tracer::new(&["cpu.regs"]);
///
/// let parent_element_id = tracer.register_element("cpu", None)
/// let child_element_id = tracer.register_element("regs", Some(parent_element_id));
///
/// tracer.trace_event(child_element_id, format_args!("{}", 42))
/// ```
#[derive(Debug)]
pub struct Tracer {
    data: RefCell<TraceData>,
}

impl Tracer {
    /// Create a new tracer, with the given trace elements enabled.
    /// If trace_file is provided, it will be used for writing trace data.
    /// Otherwise, trace data will be written to stdout.
    pub fn new<T: AsRef<str>>(enabled_trace_elements: &[T], trace_file: Option<File>) -> Self {
        let mut elements = vec![TraceElement {
            full_name: "".to_string(),
            children: HashMap::new(),
            all_children_enabled: false,
        }];

        for expr in enabled_trace_elements {
            let mut parent = ROOT_ELEMENT_ID;
            for part in expr.as_ref().split('.') {
                if let Some(existing_id) = elements[parent.0].children.get(part) {
                    parent = *existing_id;
                } else {
                    let child_id = TraceElementId(elements.len());
                    let parent_data = &mut elements[parent.0];
                    parent_data.children.insert(part.to_string(), child_id);
                    let full_name = format_trace_name(&parent_data.full_name, part);
                    elements.push(TraceElement {
                        full_name,
                        children: HashMap::new(),
                        all_children_enabled: false,
                    });
                    parent = child_id;
                }
            }
            // All children of a leaf should be enabled for tracing
            elements[parent.0].all_children_enabled = true;
        }

        let trace_writer = trace_file.map(|file| BufWriter::new(file));

        Tracer {
            data: RefCell::new(TraceData {
                elements,
                trace_writer,
            }),
        }
    }

    /// Register a new trace element
    /// Returns an ID which can be used in future trace calls
    pub fn register_element(&self, name: &str, parent: Option<TraceElementId>) -> TraceElementId {
        let parent = parent.unwrap_or(ROOT_ELEMENT_ID);
        if !parent.enabled() {
            // Child is disabled if the parent is not enabled
            return DISABLED_ELEMENT_ID;
        }

        let mut data = self.data.borrow_mut();
        // Get the next element index proactively. Needs to happen before
        // reading parent data to avoid lifetime conflicts
        let next_element_idx = data.elements.len();

        // Look up the parent
        let parent_data = &mut data.elements[parent.0];

        if let Some(existing_id) = parent_data.children.get(name) {
            // Element was pre-created, return it
            *existing_id
        } else if parent_data.all_children_enabled {
            // Element does not exist, but the parent is set to automatically
            // create and enable any additional children.
            // Create the element now.

            let child_element_id = TraceElementId(next_element_idx);
            parent_data
                .children
                .insert(name.to_string(), child_element_id);
            let full_name = format_trace_name(&parent_data.full_name, name);
            data.elements.push(TraceElement {
                full_name,
                children: HashMap::new(),
                all_children_enabled: true,
            });

            child_element_id
        } else {
            // Element does not already exist and its parent is not accepting new
            // elements. Return a disabled element.
            DISABLED_ELEMENT_ID
        }
    }

    /// Emit an event to the trace log. The message argument is the message to emit
    /// into the trace log, created using the format_args!() macro.
    pub fn trace_event(&self, element: TraceElementId, message: fmt::Arguments) {
        if !element.enabled() {
            return;
        }
        let mut data = self.data.borrow_mut();
        let data = &mut *data;
        let element_data = &data.elements[element.0];
        if let Some(writer) = &mut data.trace_writer {
            writeln!(
                writer,
                "[TRACE][{:<12}] {}",
                element_data.full_name, message
            )
            .ok();
        } else {
            println!("[TRACE][{:<12}] {}", element_data.full_name, message);
        }
    }
}

fn format_trace_name(parent_name: &str, element_name: &str) -> String {
    if parent_name.is_empty() {
        element_name.to_string()
    } else {
        format!("{}.{}", parent_name, element_name)
    }
}

/// A structure modeling a CPU register which emits a trace event when modified.
#[derive(Debug)]
pub struct TraceableReg<'t, T: Copy + Default + TraceableValue> {
    name: String,
    value: T,
    tracer: &'t Tracer,
    trace_element: TraceElementId,
}

impl<'t, T: Copy + Default + TraceableValue> TraceableReg<'t, T> {
    /// Create a new traceable register
    pub fn new(name: &str, tracer: &'t Tracer, trace_parent: TraceElementId) -> Self {
        let trace_element = tracer.register_element(name, Some(trace_parent));
        Self {
            name: name.to_string(),
            value: T::default(),
            tracer,
            trace_element,
        }
    }

    /// Get the current value of the register
    pub fn get(&self) -> T {
        self.value
    }

    /// Write a new value into the register
    pub fn set(&mut self, value: T) {
        self.value = value;
        self.tracer.trace_event(
            self.trace_element,
            format_args!("      {} = {}", self.name, TraceDisplay(&value)),
        );
    }

    /// Atomically updates the register's value through the given callback
    pub fn update(&mut self, f: impl FnOnce(T) -> T) {
        let new_value = f(self.value);
        self.set(new_value);
    }
}

impl TraceableValue for u8 {
    fn fmt_trace(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self)
    }
}
impl TraceableValue for u16 {
    fn fmt_trace(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self)
    }
}
