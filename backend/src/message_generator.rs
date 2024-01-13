use crate::api::DescriptionMessageAssessment;
use string_builder::Builder;

pub fn get_explanation_message(
    average_half_time_differential: f32,
) -> (String, DescriptionMessageAssessment) {
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
