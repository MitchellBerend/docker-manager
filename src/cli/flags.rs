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

#[derive(Debug)]
pub struct ExecFlags {
    pub detach: bool,
    pub detach_keys: String,
    pub env: Vec<String>,
    pub env_file: Vec<String>,
    pub interactive: bool,
    pub privileged: bool,
    //pub tty: bool,
    pub user: String,
    pub workdir: String,
}

impl ExecFlags {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        detach: bool,
        detach_keys: Option<String>,
        env: Option<Vec<String>>,
        env_file: Option<Vec<String>>,
        interactive: bool,
        privileged: bool,
        //tty: bool,
        user: Option<String>,
        workdir: Option<String>,
    ) -> Self {
        let detach_keys = match detach_keys {
            Some(d) => d,
            None => "".into(),
        };

        let env = match env {
            Some(e) => e,
            None => vec![],
        };

        let env_file = match env_file {
            Some(e) => e,
            None => vec![],
        };

        let user = match user {
            Some(u) => u,
            None => "".into(),
        };

        let workdir = match workdir {
            Some(w) => w,
            None => "".into(),
        };

        Self {
            detach,
            detach_keys,
            env,
            env_file,
            interactive,
            privileged,
            //tty,
            user,
            workdir,
        }
    }

    pub fn flags(&self) -> Vec<String> {
        let mut v: Vec<String> = vec![];
        if self.detach {
            v.push("-d".into());
        }

        if self.interactive {
            v.push("-i".into());
        }

        if self.privileged {
            v.push("--privileged".into());
        }

        if !self.detach_keys.is_empty() {
            v.push("--detach-keys".into());
            v.push(self.detach_keys.clone());
        }

        if !self.env.is_empty() {
            for var in &self.env {
                v.push("-e".into());
                v.push(var.clone());
            }
        }

        if !self.env_file.is_empty() {
            for file in &self.env_file {
                v.push("--env-file".into());
                v.push(file.clone());
            }
        }

        if !self.user.is_empty() {
            v.push("--user".into());
            v.push(self.user.clone());
        }

        if !self.workdir.is_empty() {
            v.push("--workdir".into());
            v.push(self.workdir.clone());
        }
        // This one is not actually useful since this program is not a tty
        // tty: bool,

        v
    }
}

pub struct ImagesFlags {
    all: bool,
    digest: bool,
    filter: String,
    format: String,
    no_trunc: bool,
    quiet: bool,
}

impl ImagesFlags {
    pub fn new(
        all: bool,
        digest: bool,
        filter: Option<String>,
        format: Option<String>,
        no_trunc: bool,
        quiet: bool,
    ) -> Self {
        let filter = match filter {
            Some(f) => f,
            None => "".into(),
        };

        let format = match format {
            Some(fo) => fo,
            None => "".into(),
        };

        Self {
            all,
            digest,
            filter,
            format,
            no_trunc,
            quiet,
        }
    }

    pub fn flags(&self) -> Vec<String> {
        let mut v: Vec<String> = vec![];
        if self.all {
            v.push("-".into());
        }

        if self.digest {
            v.push("--digest".into());
        }

        if !self.filter.is_empty() {
            v.push("--filter".into());
            v.push(self.filter.clone());
        }

        if !self.format.is_empty() {
            v.push("--format".into());
            v.push(self.format.clone());
        }

        if self.no_trunc {
            v.push("--no-trunc".into());
        }

        if self.quiet {
            v.push("--quiet".into());
        }

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
