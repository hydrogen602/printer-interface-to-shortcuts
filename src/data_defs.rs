pub mod printer_tool {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase", tag = "command")]
    pub enum Tool {
        #[serde(rename = "target")]
        Target { targets: Targets },
        #[serde(rename = "extrude")]
        Extrude {
            /// in mm, pos is extrude, neg is retract
            amount: f64,
            /// in mm/min
            #[serde(skip_serializing_if = "Option::is_none")]
            speed: Option<f64>,
        },
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Targets {
        pub tool0: i64,
    }
}

pub mod printer_move {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase", tag = "command")]
    pub enum PrinterMove {
        #[serde(rename = "jog")]
        Move {
            #[serde(skip_serializing_if = "Option::is_none")]
            x: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            y: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            z: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            absolute: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            speed: Option<f64>,
        },
        #[serde(rename = "home")]
        Home { axes: Vec<HomeAxis> },
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum HomeAxis {
        X,
        Y,
        Z,
    }

    impl PrinterMove {
        pub fn home_all() -> Self {
            Self::Home {
                axes: vec![HomeAxis::X, HomeAxis::Y, HomeAxis::Z],
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_home() {
            let home = PrinterMove::home_all();
            assert_eq!(
                home,
                PrinterMove::Home {
                    axes: vec![HomeAxis::X, HomeAxis::Y, HomeAxis::Z]
                }
            );
            let cmd = serde_json::to_string(&home).unwrap();
            assert_eq!(cmd, r#"{"command":"home","axes":["x","y","z"]}"#);
        }

        #[test]
        fn test_move() {
            let move_ = PrinterMove::Move {
                x: Some(10.0),
                y: None,
                z: None,
                absolute: Some(true),
                speed: None,
            };
            let cmd = serde_json::to_string(&move_).unwrap();
            assert_eq!(cmd, r#"{"command":"jog","x":10.0,"absolute":true}"#);
        }
    }
}

pub mod printer_state {
    use serde::{Deserialize, Serialize};
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PrinterState {
        pub sd: Sd,
        pub state: State,
        pub temperature: Temperature,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Sd {
        pub ready: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct State {
        pub error: String,
        pub flags: Flags,
        pub text: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Flags {
        pub cancelling: bool,
        pub closed_or_error: bool,
        pub error: bool,
        pub finishing: bool,
        pub operational: bool,
        pub paused: bool,
        pub pausing: bool,
        pub printing: bool,
        pub ready: bool,
        pub resuming: bool,
        pub sd_ready: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Temperature {
        pub bed: Bed,
        pub tool0: Tool0,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Bed {
        pub actual: f64,
        pub offset: i64,
        pub target: f64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Tool0 {
        pub actual: f64,
        pub offset: i64,
        pub target: f64,
    }
}

pub mod printer_job_state {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct JobState {
        pub job: Job,
        pub progress: Progress,
        pub state: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Job {
        pub file: File,
        pub estimated_print_time: Option<f64>,
        pub average_print_time: Option<f64>,
        pub filament: Option<Filament>,
        pub last_print_time: Option<f64>,
        pub user: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct File {
        pub name: Option<String>,
        pub origin: Option<String>,
        pub size: Option<i64>,
        pub date: Option<i64>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Filament {
        pub tool0: Tool0,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Tool0 {
        pub length: f64,
        pub volume: f64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Progress {
        pub completion: Option<f64>,
        pub filepos: Option<i64>,
        pub print_time: Option<i64>,
        pub print_time_left: Option<i64>,
    }
}

pub mod printer_job_action {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase", tag = "command")]
    pub enum JobAction {
        Start,
        Cancel,
        Restart,
        Pause { action: PauseAction },
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum PauseAction {
        Pause,
        Resume,
        Toggle,
    }

    #[test]
    fn test_job_change() {
        let move_ = JobAction::Start;
        let cmd = serde_json::to_string(&move_).unwrap();
        assert_eq!(cmd, r#"{"command":"start"}"#);

        let move_ = JobAction::Cancel;
        let cmd = serde_json::to_string(&move_).unwrap();
        assert_eq!(cmd, r#"{"command":"cancel"}"#);

        let move_ = JobAction::Restart;
        let cmd = serde_json::to_string(&move_).unwrap();
        assert_eq!(cmd, r#"{"command":"restart"}"#);

        let move_ = JobAction::Pause {
            action: PauseAction::Pause,
        };
        let cmd = serde_json::to_string(&move_).unwrap();
        assert_eq!(cmd, r#"{"command":"pause","action":"pause"}"#);

        let move_ = JobAction::Pause {
            action: PauseAction::Resume,
        };
        let cmd = serde_json::to_string(&move_).unwrap();
        assert_eq!(cmd, r#"{"command":"pause","action":"resume"}"#);

        let move_ = JobAction::Pause {
            action: PauseAction::Toggle,
        };
        let cmd = serde_json::to_string(&move_).unwrap();
        assert_eq!(cmd, r#"{"command":"pause","action":"toggle"}"#);
    }
}
