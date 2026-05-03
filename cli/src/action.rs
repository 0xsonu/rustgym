/// Identifies a screen in the navigation stack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScreenId {
    Login,
    QuestList,
    QuestDetail { slug: String },
    LevelDetail { quest_slug: String, level_slug: String },
    TaskDetail { slug: String },
    Submit { slug: String },
}

/// Represents an API call to be dispatched in the background.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiCall {
    Login { email: String, password: String },
    FetchQuests,
    FetchQuestDetail { slug: String },
    FetchLevelDetail { quest_slug: String, level_slug: String },
    FetchTaskDetail { slug: String },
    SubmitSolution { slug: String, code: String },
}

/// Actions returned by screen update methods to drive the application state machine.
#[derive(Debug, Clone)]
pub enum Action {
    /// No-op, nothing to do.
    None,
    /// Push a new screen onto the navigation stack.
    Push(ScreenId),
    /// Pop the current screen (go back).
    Pop,
    /// Replace the current screen.
    Replace(ScreenId),
    /// Fire an API request in the background.
    ApiRequest(ApiCall),
    /// Suspend TUI and launch editor for the given task slug.
    LaunchEditor {
        slug: String,
        starter_code: String,
        description_md: String,
    },
    /// Quit the application.
    Quit,
    /// Show a transient error message.
    ShowError(String),
    /// Store authentication token and update auth state.
    SetAuth { token: String, username: String, xp: i32 },
    /// Log out: clear token and navigate to login.
    Logout,
    /// Multiple actions to execute in sequence.
    Batch(Vec<Action>),
}
