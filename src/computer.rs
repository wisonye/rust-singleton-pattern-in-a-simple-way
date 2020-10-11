/// The struct which applied "Builder" and "Singleton" pattern
pub struct Computer {
    cpu_vender: String,
    cpu_model: String,
    memory_vender: String,
    memory_size_in_gb: u16,
}

/// Singleton mutable instance
static mut COMPUTER_SINGLE_INSTANCE: Option<Computer> = None;

impl std::fmt::Debug for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = f
            .debug_struct("\n[ Computer ]")
            .field("CPU Vender", &self.cpu_vender)
            .field("CPU Model", &self.cpu_model)
            .field("Memory Vender", &self.memory_vender);

        // No need to call finish, as we need to add one more customized debug info
        let _ = f.write_fmt(format_args!("    Memory Size: {} GB", self.memory_size_in_gb));

        // Add the missing `}`, as we don't call `debug_struct().finished()`
        f.write_str("\n}\n")
    }
}

///
impl Computer {
    /// Singleton constructor
    pub fn get_computer() -> &'static mut Self {
        unsafe {
            if COMPUTER_SINGLE_INSTANCE.is_none() {
                COMPUTER_SINGLE_INSTANCE = Some(Computer {
                    cpu_vender: "".to_owned(),
                    cpu_model: "".to_owned(),
                    memory_vender: "".to_owned(),
                    memory_size_in_gb: 0,
                });
            }

            return COMPUTER_SINGLE_INSTANCE.as_mut().unwrap();
        }
    }

    ///
    pub fn add_cpu(&mut self, vender: &str, model: &str) -> &mut Self {
        self.cpu_vender = vender.to_owned();
        self.cpu_model = model.to_owned();
        self
    }

    ///
    pub fn add_memory(&mut self, vender: &str, size_in_gb: u16) -> &mut Self {
        self.memory_vender = vender.to_owned();
        self.memory_size_in_gb = size_in_gb;
        self
    }
}
