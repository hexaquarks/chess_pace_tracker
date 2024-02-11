use crate::service_intermediary::DescriptionMessageAssessment;

const INVALID_TIME_DESCRIPTION_PLACEHOLDER_MSG: &str =
    "The time value was not computed. Check the errors panel for more information.";

// Everything is a string for proper serialization to frontend
pub struct InsightsPanelProps {
    pub average_time: String,
    pub explanation_message: (String, DescriptionMessageAssessment),
    pub win_ratio: String,
}

pub struct MessageContext {
    average_time: f32,
}

impl MessageContext {
    pub fn new(average_time_opt: Option<f32>) -> Self {
        Self {
            average_time: average_time_opt.unwrap_or(std::f32::MAX),
        }
    }

    pub fn generate_message(&self) -> (String, DescriptionMessageAssessment) {
        if self.average_time == std::f32::MAX {
            return (
                INVALID_TIME_DESCRIPTION_PLACEHOLDER_MSG.to_string(),
                DescriptionMessageAssessment::Negative,
            );
        }

        let (status_message, assessment) = match self.average_time.partial_cmp(&0.0).unwrap() {
            std::cmp::Ordering::Less => (
                "behind your opponent by",
                DescriptionMessageAssessment::Negative,
            ),
            std::cmp::Ordering::Equal => (
                "equal in time to your opponent",
                DescriptionMessageAssessment::Neutral,
            ),
            std::cmp::Ordering::Greater => (
                "ahead of your opponent by",
                DescriptionMessageAssessment::Positive,
            ),
        };

        let time_message = if self.average_time != std::f32::MAX {
            format!(" {:.2} seconds", self.average_time.abs())
        } else {
            String::new()
        };

        let message = format!(
            "On average, you are {}{} at half time in the games.",
            status_message, time_message
        );

        (message, assessment)
    }
}

pub fn get_average_time_as_formatted_string(
    average_half_time_differential_opt: Option<f32>,
) -> String {
    if average_half_time_differential_opt.is_none() {
        return String::from("Undefined ");
    } else {
        return average_half_time_differential_opt.unwrap().to_string();
    }
}

pub fn get_feedback_message(
    average_half_time_differential_opt: Option<f32>,
) -> (String, DescriptionMessageAssessment) {
    let context = MessageContext::new(average_half_time_differential_opt);
    context.generate_message()
}

pub fn get_win_ratio_as_formatted_string(player_win_rate_in_fetched_games: f32) -> String {
    format!("{:.2}", player_win_rate_in_fetched_games)
}

pub fn get_insights(
    average_half_time_differential_opt: Option<f32>,
    player_win_rate_in_fetched_games: f32,
) -> InsightsPanelProps {
    InsightsPanelProps {
        average_time: get_average_time_as_formatted_string(average_half_time_differential_opt),
        explanation_message: get_feedback_message(average_half_time_differential_opt),
        win_ratio: get_win_ratio_as_formatted_string(player_win_rate_in_fetched_games),
    }
}
