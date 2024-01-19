use crate::api::DescriptionMessageAssessment;
use string_builder::Builder;

const INVALID_TIME_DESCRIPTION_PLACEHOLDER_MSG: &str =
    "The time value was not computed. Check the errors panel for more information.";

pub fn get_average_time_string_fmt(average_half_time_differential_opt: Option<f32>) -> String {
    if average_half_time_differential_opt.is_none() {
        return String::from("Undefined ");
    } else {
        return average_half_time_differential_opt.unwrap().to_string();
    }
}

pub fn get_explanation_message(
    average_half_time_differential_opt: Option<f32>,
) -> (String, DescriptionMessageAssessment) {
    // Edge case, no games were used in time computation
    if average_half_time_differential_opt.is_none() {
        return (
            INVALID_TIME_DESCRIPTION_PLACEHOLDER_MSG.to_string(),
            DescriptionMessageAssessment::Negative,
        );
    }

    let average_half_time_differential = average_half_time_differential_opt.unwrap();

    let mut message_assessment;
    let mut message_builder = Builder::default();
    message_builder.append("On average, you are ");

    let time_message = format!("{:.2} seconds ", average_half_time_differential.abs());
    if average_half_time_differential < 0.0 {
        message_builder.append("behind your opponent by ");
        message_builder.append(time_message);

        message_assessment = DescriptionMessageAssessment::Negative;
    } else if average_half_time_differential == 0.0 {
        message_builder.append("equal in time to your opponent ");

        message_assessment = DescriptionMessageAssessment::Neutral;
    } else {
        message_builder.append("ahead your opponent by ");
        message_builder.append(time_message);

        message_assessment = DescriptionMessageAssessment::Positive;
    }
    message_builder.append("at half time in the games.");

    (message_builder.string().unwrap(), message_assessment)
}
