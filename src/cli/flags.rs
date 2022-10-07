pub struct LogsFlags {
    pub details: bool,
    pub follow: bool,
    pub since: String,
    pub tail: String,
    pub timestamps: bool,
    pub until: String,
}

impl LogsFlags {
    pub fn new(
        details: bool,
        follow: bool,
        since: Option<String>,
        tail: Option<String>,
        timestamps: bool,
        until: Option<String>,
    ) -> Self {
        let since: String = match since {
            Some(since) => since,
            None => "".into(),
        };
        let tail: String = match tail {
            Some(tail) => tail,
            None => "".into(),
        };
        let until: String = match until {
            Some(until) => until,
            None => "".into(),
        };

        Self {
            details,
            follow,
            since,
            tail,
            timestamps,
            until,
        }
    }

    pub fn flags(&self) -> Vec<String> {
        let mut v: Vec<String> = vec![];
        if self.details {
            v.push("-d".into())
        }

        // special case which is handled in the run_logs function
        //if self.follow {
        //    v.push("-f".into())

        if !self.since.is_empty() {
            v.push("--since".into());
            v.push(self.since.clone());
        };

        if !self.tail.is_empty() {
            v.push("--tail".into());
            v.push(self.tail.clone());
        };

        if self.timestamps {
            v.push("--timestamps".into())
        }

        if !self.until.is_empty() {
            v.push("--until".into());
            v.push(self.until.clone());
        };

        v
    }
}

pub struct PsFlags {
    pub all: bool,
    pub filter: String,
    pub format: String,
    pub last: bool,
    pub latests: bool,
    pub no_trunc: bool,
    pub quiet: bool,
    pub size: bool,
}

#[allow(clippy::too_many_arguments)]
impl PsFlags {
    pub fn new(
        all: bool,
        filter: Option<String>,
        format: Option<String>,
        last: bool,
        latests: bool,
        no_trunc: bool,
        quiet: bool,
        size: bool,
    ) -> Self {
        let filter: String = match filter {
            Some(filter) => filter,
            None => "".into(),
        };
        let format: String = match format {
            Some(format) => format,
            None => "".into(),
        };

        Self {
            all,
            filter,
            format,
            last,
            latests,
            no_trunc,
            quiet,
            size,
        }
    }

    pub fn flags(&self) -> Vec<String> {
        let mut v: Vec<String> = vec![];
        if self.all {
            v.push("-a".into())
        }

        if !self.filter.is_empty() {
            v.push("--filter".into());
            v.push(self.filter.clone());
        };

        if !self.format.is_empty() {
            v.push("--format".into());
            v.push(self.format.clone());
        };

        if self.last {
            v.push("--last".into())
        }

        if self.last {
            v.push("--last".into())
        }

        if self.latests {
            v.push("--latests".into())
        }
        if self.no_trunc {
            v.push("--no_trunc".into())
        }
        if self.quiet {
            v.push("--quiet".into())
        }
        if self.size {
            v.push("--size".into())
        }

        v
    }
}
