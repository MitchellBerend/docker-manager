use defaultdict::DefaultHashMap;

const HOSTNAME: &str = "HOSTNAME";
const OFFSET: usize = 3;

#[derive(Debug)]
pub struct Parser {
    headers: Vec<String>,
    header_spacing: DefaultHashMap<String, usize>,

    internal: DefaultHashMap<String, Vec<Vec<String>>>,
}

impl Parser {
    pub fn from_command_results(log: String) -> Self {
        let mut internal: DefaultHashMap<String, Vec<Vec<String>>> = DefaultHashMap::new();
        let mut header_spacing: DefaultHashMap<String, usize> = DefaultHashMap::new();
        let mut headers: Vec<String> = vec![];

        let mut node: Option<String> = None;

        // The arms in this if block only execute the same action, they do check different things.
        #[allow(clippy::if_same_then_else)]
        for line in log.split('\n') {
            if line.contains("CONTAINER") {
                let headers_iter: Vec<String> = line
                    .split("  ")
                    .filter(|item| !item.is_empty())
                    .map(|item| String::from(item.trim()))
                    .collect();
                if headers.len() < headers_iter.len() {
                    headers = headers_iter;
                }
                continue;
            } else if line.contains("not found") {
                continue;
            } else if line.is_empty() {
                continue;
            } else if line.contains("Error") {
                continue;
            } else if !line.contains(' ') {
                node = Some(String::from(line))
            } else if let Some(node) = &node {
                let mut placeholder: Vec<String> = vec![];
                for item in line.split("  ").filter(|item| !item.is_empty()) {
                    placeholder.push(String::from(item.trim()));
                }
                internal.get_mut(node).push(placeholder);
            }
        }

        headers.insert(0, HOSTNAME.to_string());
        header_spacing.insert(String::from(HOSTNAME), String::from(HOSTNAME).len());

        Self {
            headers,
            header_spacing,
            internal,
        }
    }

    pub fn print(&mut self) {
        for host in self.internal.keys() {
            let lines = self.internal.get(host);
            let number = std::cmp::max(
                host.len() + OFFSET,
                self.header_spacing[&String::from(HOSTNAME)],
            );
            self.header_spacing.insert(String::from(HOSTNAME), number);
            for line in lines {
                for (header_index, item) in line.iter().enumerate() {
                    if let Some(header) = self.headers.get(header_index + 1) {
                        let number =
                            std::cmp::max(item.len() + OFFSET, *self.header_spacing.get(header));
                        self.header_spacing.insert(header.to_owned(), number);
                    }
                }
            }
        }

        let mut headers = String::new();

        for header in &self.headers {
            let spacing: usize = *self.header_spacing.get(header);
            headers.push_str(header);
            if spacing >= header.len() {
                let offset = spacing - header.len();
                for _ in 0..offset {
                    headers.push(' ');
                }
            }
            headers.push('\t');
        }

        let mut body = String::new();

        for host in self.internal.keys() {
            let lines = self.internal.get(host);
            for line in lines {
                let host_spacing: usize = *self.header_spacing.get(HOSTNAME);
                let mut _body: String = String::from(host);
                if host_spacing >= host.len() {
                    let offset = host_spacing - host.len();
                    for _ in 0..offset {
                        _body.push(' ');
                    }
                }
                _body.push('\t');

                for (index, item) in line.iter().enumerate() {
                    if let Some(header) = self.headers.get(index + 1) {
                        let spacing = *self.header_spacing.get(header);
                        _body.push_str(item);

                        if spacing >= item.len() {
                            let offset = spacing - item.len();
                            for _ in 0..offset {
                                _body.push(' ');
                            }
                        }
                        _body.push('\t');
                    }
                }
                body.push_str(&format!("{}\n", _body));
            }
        }

        println!("{}\n{}\n", headers, body);
    }
}
