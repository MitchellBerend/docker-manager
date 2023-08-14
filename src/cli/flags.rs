pub struct LogsFlags<'a> {
    pub details: bool,
    pub follow: bool,
    pub since: &'a str,
    pub tail: &'a str,
    pub timestamps: bool,
    pub until: &'a str,
}

impl<'a> LogsFlags<'a> {
    pub fn new(
        details: bool,
        follow: bool,
        since: &'a Option<&'a str>,
        tail: &'a Option<&'a str>,
        timestamps: bool,
        until: &'a Option<&'a str>,
    ) -> Self {
        let _since: &str = match since {
            Some(since) => since,
            None => "",
        };
        let tail: &str = match tail {
            Some(tail) => tail,
            None => "",
        };
        let until: &str = match until {
            Some(until) => until,
            None => "",
        };

        Self {
            details,
            follow,
            since: _since,
            tail,
            timestamps,
            until,
        }
    }

    pub fn flags(&self) -> Vec<&str> {
        let mut v: Vec<&str> = vec![];
        if self.details {
            v.push("-d")
        }

        // special case which is handled in the run_logs function
        //if self.follow {
        //    v.push("-f".into())

        if !self.since.is_empty() {
            v.push("--since");
            v.push(self.since);
        };

        if !self.tail.is_empty() {
            v.push("--tail");
            v.push(self.tail);
        };

        if self.timestamps {
            v.push("--timestamps")
        }

        if !self.until.is_empty() {
            v.push("--until");
            v.push(self.until);
        };

        v
    }
}

#[derive(Debug)]
pub struct ExecFlags<'a> {
    pub detach: bool,
    pub detach_keys: &'a str,
    pub env: Vec<&'a str>,
    pub env_file: Vec<&'a str>,
    pub interactive: bool,
    pub privileged: bool,
    //pub tty: bool,
    pub user: &'a str,
    pub workdir: &'a str,
}

impl<'a> ExecFlags<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        detach: bool,
        detach_keys: &'a Option<&str>,
        env: Option<Vec<&'a str>>,
        env_file: Option<Vec<&'a str>>,
        interactive: bool,
        privileged: bool,
        //tty: bool,
        user: &'a Option<&'a str>,
        workdir: &'a Option<&'a str>,
    ) -> Self {
        let detach_keys = match &detach_keys {
            Some(d) => d,
            None => "",
        };

        let env = match env {
            Some(e) => e,
            None => vec![],
        };

        let env_file = match env_file {
            Some(e) => e,
            None => vec![],
        };

        let user = match &user {
            Some(u) => u,
            None => "",
        };

        let workdir = match &workdir {
            Some(w) => w,
            None => "",
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

    pub fn flags(&self) -> Vec<&str> {
        let mut v: Vec<&str> = vec![];
        if self.detach {
            v.push("-d");
        }

        if self.interactive {
            v.push("-i");
        }

        if self.privileged {
            v.push("--privileged");
        }

        if !self.detach_keys.is_empty() {
            v.push("--detach-keys");
            v.push(self.detach_keys);
        }

        if !self.env.is_empty() {
            for var in &self.env {
                v.push("-e");
                v.push(var);
            }
        }

        if !self.env_file.is_empty() {
            for file in &self.env_file {
                v.push("--env-file");
                v.push(file);
            }
        }

        if !self.user.is_empty() {
            v.push("--user");
            v.push(self.user);
        }

        if !self.workdir.is_empty() {
            v.push("--workdir");
            v.push(self.workdir);
        }
        // This one is not actually useful since this program is not a tty
        // tty: bool,

        v
    }
}

pub struct ImagesFlags<'a> {
    all: bool,
    digest: bool,
    filter: &'a str,
    format: &'a str,
    no_trunc: bool,
    quiet: bool,
}

impl<'a> ImagesFlags<'a> {
    pub fn new(
        all: bool,
        digest: bool,
        filter: &'a Option<&'a str>,
        format: &'a Option<&'a str>,
        no_trunc: bool,
        quiet: bool,
    ) -> Self {
        let filter = match &filter {
            Some(f) => f,
            None => "",
        };

        let format = match &format {
            Some(fo) => fo,
            None => "",
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

    pub fn flags(&self) -> Vec<&str> {
        let mut v: Vec<&str> = vec![];
        if self.all {
            v.push("-");
        }

        if self.digest {
            v.push("--digest");
        }

        if !self.filter.is_empty() {
            v.push("--filter");
            v.push(self.filter);
        }

        if !self.format.is_empty() {
            v.push("--format");
            v.push(self.format);
        }

        if self.no_trunc {
            v.push("--no-trunc");
        }

        if self.quiet {
            v.push("--quiet");
        }

        v
    }
}

pub struct PsFlags<'a> {
    pub all: bool,
    pub filter: &'a str,
    pub format: &'a str,
    pub last: bool,
    pub latests: bool,
    pub no_trunc: bool,
    pub quiet: bool,
    pub size: bool,
}

#[allow(clippy::too_many_arguments)]
impl<'a> PsFlags<'a> {
    pub fn new(
        all: bool,
        filter: &'a Option<&'a str>,
        format: &'a Option<&'a str>,
        last: bool,
        latests: bool,
        no_trunc: bool,
        quiet: bool,
        size: bool,
    ) -> Self {
        let filter: &str = match filter {
            Some(filter) => filter,
            None => "",
        };
        let format: &str = match format {
            Some(format) => format,
            None => "",
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

    pub fn flags(&self) -> Vec<&str> {
        let mut v: Vec<&str> = vec![];
        if self.all {
            v.push("-a")
        }

        if !self.filter.is_empty() {
            v.push("--filter");
            v.push(self.filter);
        };

        if !self.format.is_empty() {
            v.push("--format");
            v.push(self.format);
        };

        if self.last {
            v.push("--last")
        }

        if self.last {
            v.push("--last")
        }

        if self.latests {
            v.push("--latests")
        }
        if self.no_trunc {
            v.push("--no_trunc")
        }
        if self.quiet {
            v.push("--quiet")
        }
        if self.size {
            v.push("--size")
        }

        v
    }
}

pub struct RmFlags {
    pub force: bool,
    pub volume: bool,
}

impl RmFlags {
    pub fn new(force: bool, volume: bool) -> Self {
        Self { force, volume }
    }

    pub fn flags(&self) -> Vec<&'static str> {
        let mut v: Vec<&str> = vec![];

        if self.force {
            v.push("-f")
        }

        if self.volume {
            v.push("-v")
        }

        v
    }
}
