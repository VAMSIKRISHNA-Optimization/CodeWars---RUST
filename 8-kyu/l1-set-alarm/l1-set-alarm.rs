fn set_alarm(employed: bool, vacation: bool) -> bool {
        if employed && !vacation { true } else { false }
}