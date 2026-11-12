//! Poll models.
//!
//! Polls are attached to messages and contain a question, a set of answers and
//! optional vote results. The result counts are only final once Discord marks
//! the poll as finalized.

use std::fmt;
use std::time::SystemTime;

use data::{Object, Value};
use enums::PollLayoutType;

use crate::PartialEmoji;
use crate::cast::{self, CastError};

/// The display data for a poll question or answer.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PollMedia {
    /// The displayed text, if any.
    pub text: Option<String>,
    /// The attached emoji, if any.
    pub emoji: Option<PartialEmoji>,
}

impl PollMedia {
    /// Creates media with the given text.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            emoji: None,
        }
    }

    /// Creates media containing only an emoji.
    #[must_use]
    pub fn from_emoji(emoji: PartialEmoji) -> Self {
        Self {
            text: None,
            emoji: Some(emoji),
        }
    }

    /// Sets the media's emoji.
    #[must_use]
    pub fn with_emoji(mut self, emoji: PartialEmoji) -> Self {
        self.emoji = Some(emoji);
        self
    }

    /// Decodes poll media from a Discord poll media object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a field has an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "poll_media")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        let emoji_path = cast::field_path(path, "emoji");
        let emoji = cast::optional_object(object, path, "emoji")?
            .map(|emoji| PartialEmoji::from_object_at(emoji, &emoji_path))
            .transpose()?;
        Ok(Self {
            text: cast::optional_string(object, path, "text")?.map(str::to_owned),
            emoji,
        })
    }

    /// Returns the displayed text, if any.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Returns `true` if the media has neither text nor an emoji.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_none() && self.emoji.is_none()
    }
}

impl From<&str> for PollMedia {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl From<String> for PollMedia {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl fmt::Display for PollMedia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.emoji, &self.text) {
            (Some(emoji), Some(text)) => write!(f, "{emoji} {text}"),
            (Some(emoji), None) => emoji.fmt(f),
            (None, Some(text)) => f.write_str(text),
            (None, None) => Ok(()),
        }
    }
}

/// An answer in a poll.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PollAnswer {
    /// The answer's ID within the poll.
    pub answer_id: u32,
    /// The answer's display data.
    pub media: PollMedia,
    /// The number of votes for this answer.
    pub vote_count: u32,
    /// Whether the current user voted for this answer.
    pub self_voted: bool,
}

impl PollAnswer {
    /// Creates an answer with no votes.
    #[must_use]
    pub fn new(answer_id: u32, media: impl Into<PollMedia>) -> Self {
        Self {
            answer_id,
            media: media.into(),
            vote_count: 0,
            self_voted: false,
        }
    }

    /// Sets the answer's vote count.
    #[must_use]
    pub fn with_vote_count(mut self, vote_count: u32) -> Self {
        self.vote_count = vote_count;
        self
    }

    /// Sets whether the current user voted for this answer.
    #[must_use]
    pub fn with_self_voted(mut self, self_voted: bool) -> Self {
        self.self_voted = self_voted;
        self
    }

    /// Decodes an answer from a Discord poll answer object.
    ///
    /// Vote counts are carried by [`PollResults`], so the returned answer has
    /// no votes of its own.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "poll_answer")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        let media_path = cast::field_path(path, "poll_media");
        let media = PollMedia::from_object_at(
            cast::required_object(object, path, "poll_media")?,
            &media_path,
        )?;
        Ok(Self {
            answer_id: cast::required_u32(object, path, "answer_id")?,
            media,
            vote_count: 0,
            self_voted: false,
        })
    }

    /// Returns the answer's displayed text, if any.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.media.text()
    }

    /// Returns the answer's emoji, if any.
    #[must_use]
    pub fn emoji(&self) -> Option<&PartialEmoji> {
        self.media.emoji.as_ref()
    }
}

impl fmt::Display for PollAnswer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.media.fmt(f)
    }
}

/// The number of votes for a single answer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PollAnswerCount {
    /// The answer's ID.
    pub id: u32,
    /// The number of votes for the answer.
    pub count: u32,
    /// Whether the current user voted for the answer.
    pub me_voted: bool,
}

impl PollAnswerCount {
    /// Creates a count for the given answer.
    #[must_use]
    pub const fn new(id: u32, count: u32) -> Self {
        Self {
            id,
            count,
            me_voted: false,
        }
    }

    /// Sets whether the current user voted for the answer.
    #[must_use]
    pub const fn with_me_voted(mut self, me_voted: bool) -> Self {
        self.me_voted = me_voted;
        self
    }

    /// Decodes an answer count from a Discord poll answer count object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "poll_answer_count")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        Ok(Self {
            id: cast::required_u32(object, path, "id")?,
            count: cast::required_u32(object, path, "count")?,
            me_voted: cast::optional_bool(object, path, "me_voted", false)?,
        })
    }
}

/// The results of a poll.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PollResults {
    /// Whether the results are final.
    pub is_finalized: bool,
    /// The vote counts for each answer.
    pub answer_counts: Vec<PollAnswerCount>,
}

impl PollResults {
    /// Creates an unfinalized result set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether the results are final.
    #[must_use]
    pub fn with_is_finalized(mut self, is_finalized: bool) -> Self {
        self.is_finalized = is_finalized;
        self
    }

    /// Sets the vote counts for each answer.
    #[must_use]
    pub fn with_answer_counts(mut self, answer_counts: Vec<PollAnswerCount>) -> Self {
        self.answer_counts = answer_counts;
        self
    }

    /// Appends an answer count.
    #[must_use]
    pub fn add_count(mut self, count: PollAnswerCount) -> Self {
        self.answer_counts.push(count);
        self
    }

    /// Decodes poll results from a Discord poll results object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "poll_results")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        Ok(Self {
            is_finalized: cast::required_bool(object, path, "is_finalized")?,
            answer_counts: cast::optional_array_of(
                object,
                path,
                "answer_counts",
                PollAnswerCount::from_value_at,
            )?
            .unwrap_or_default(),
        })
    }

    /// Returns the count for an answer, if present.
    #[must_use]
    pub fn get_count(&self, answer_id: u32) -> Option<&PollAnswerCount> {
        self.answer_counts
            .iter()
            .find(|count| count.id == answer_id)
    }

    /// Returns the total number of votes across every answer.
    #[must_use]
    pub fn total_votes(&self) -> u64 {
        self.answer_counts
            .iter()
            .map(|count| u64::from(count.count))
            .sum()
    }
}

/// A poll attached to a message.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Poll {
    /// The poll's question.
    pub question: PollMedia,
    /// The poll's answers.
    pub answers: Vec<PollAnswer>,
    /// When the poll expires, if known.
    pub expiry: Option<SystemTime>,
    /// Whether users may select more than one answer.
    pub allow_multiselect: bool,
    /// The poll's layout.
    pub layout_type: PollLayoutType,
    /// The poll's results, if available.
    pub results: Option<PollResults>,
}

impl Poll {
    /// Creates a poll with the given question and no answers.
    #[must_use]
    pub fn new(question: impl Into<PollMedia>) -> Self {
        Self {
            question: question.into(),
            ..Self::default()
        }
    }

    /// Decodes a poll from a Discord poll object.
    ///
    /// # Errors
    ///
    /// Returns [`CastError`] when a required field is missing or a field has
    /// an unexpected type.
    pub fn from_value(value: &Value) -> Result<Self, CastError> {
        Self::from_value_at(value, "poll")
    }

    pub(crate) fn from_value_at(value: &Value, path: &str) -> Result<Self, CastError> {
        let object = cast::object(value, path)?;
        Self::from_object_at(object, path)
    }

    pub(crate) fn from_object_at(object: &Object, path: &str) -> Result<Self, CastError> {
        let question_path = cast::field_path(path, "question");
        let question = PollMedia::from_object_at(
            cast::required_object(object, path, "question")?,
            &question_path,
        )?;

        let results_path = cast::field_path(path, "results");
        let results = cast::optional_object(object, path, "results")?
            .map(|results| PollResults::from_object_at(results, &results_path))
            .transpose()?;

        Ok(Self {
            question,
            answers: cast::optional_array_of(object, path, "answers", PollAnswer::from_value_at)?
                .unwrap_or_default(),
            expiry: cast::optional_timestamp(object, path, "expiry")?,
            allow_multiselect: cast::optional_bool(object, path, "allow_multiselect", false)?,
            layout_type: cast::optional_enum(
                object,
                path,
                "layout_type",
                "a poll layout type",
                PollLayoutType::from_value,
            )?
            .unwrap_or_default(),
            results,
        })
    }

    /// Sets when the poll expires.
    #[must_use]
    pub fn with_expiry(mut self, expiry: SystemTime) -> Self {
        self.expiry = Some(expiry);
        self
    }

    /// Sets whether users may select more than one answer.
    #[must_use]
    pub fn with_allow_multiselect(mut self, allow_multiselect: bool) -> Self {
        self.allow_multiselect = allow_multiselect;
        self
    }

    /// Sets the poll's layout.
    #[must_use]
    pub fn with_layout_type(mut self, layout_type: PollLayoutType) -> Self {
        self.layout_type = layout_type;
        self
    }

    /// Sets the poll's answers.
    #[must_use]
    pub fn with_answers(mut self, answers: Vec<PollAnswer>) -> Self {
        self.answers = answers;
        self
    }

    /// Appends an answer.
    #[must_use]
    pub fn add_answer(mut self, answer: PollAnswer) -> Self {
        self.answers.push(answer);
        self
    }

    /// Sets the poll's results.
    #[must_use]
    pub fn with_results(mut self, results: PollResults) -> Self {
        self.results = Some(results);
        self
    }

    /// Returns the poll's question text, if any.
    #[must_use]
    pub fn question_text(&self) -> Option<&str> {
        self.question.text()
    }

    /// Returns the poll's answers.
    #[must_use]
    pub fn answers(&self) -> &[PollAnswer] {
        &self.answers
    }

    /// Returns the answer with the given ID, if it exists.
    #[must_use]
    pub fn get_answer(&self, answer_id: u32) -> Option<&PollAnswer> {
        self.answers
            .iter()
            .find(|answer| answer.answer_id == answer_id)
    }

    /// Returns when the poll expires, if known.
    #[must_use]
    pub const fn expires_at(&self) -> Option<SystemTime> {
        self.expiry
    }

    /// Returns `true` if Discord has finalized the poll.
    #[must_use]
    pub fn is_finalized(&self) -> bool {
        self.results
            .as_ref()
            .is_some_and(|results| results.is_finalized)
    }

    /// British spelling of [`Poll::is_finalized`].
    #[must_use]
    pub fn is_finalised(&self) -> bool {
        self.is_finalized()
    }

    /// Returns the total number of votes.
    ///
    /// Finalized results are used when available; otherwise the per-answer
    /// counts are summed.
    #[must_use]
    pub fn total_votes(&self) -> u64 {
        self.results.as_ref().map_or_else(
            || {
                self.answers
                    .iter()
                    .map(|answer| u64::from(answer.vote_count))
                    .sum()
            },
            PollResults::total_votes,
        )
    }

    /// Returns the ID of the answer with the most votes, once the poll has
    /// finalized.
    #[must_use]
    pub fn victor_answer_id(&self) -> Option<u32> {
        let results = self
            .results
            .as_ref()
            .filter(|results| results.is_finalized)?;
        results
            .answer_counts
            .iter()
            .max_by_key(|count| count.count)
            .map(|count| count.id)
    }

    /// Returns the answer with the most votes, once the poll has finalized.
    #[must_use]
    pub fn victor_answer(&self) -> Option<&PollAnswer> {
        self.get_answer(self.victor_answer_id()?)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use data::json;
    use enums::PollLayoutType;

    use super::{Poll, PollAnswer, PollAnswerCount, PollMedia, PollResults};
    use crate::PartialEmoji;

    #[test]
    fn poll_media_text_and_emoji() {
        let text = PollMedia::new("Favourite?");
        assert_eq!(text.text(), Some("Favourite?"));
        assert!(!text.is_empty());
        assert_eq!(text.to_string(), "Favourite?");

        let emoji = PollMedia::from_emoji(PartialEmoji::unicode("🎉"));
        assert_eq!(emoji.text(), None);
        assert!(!emoji.is_empty());
        assert_eq!(emoji.to_string(), "🎉");

        let both = PollMedia::new("Party").with_emoji(PartialEmoji::unicode("🎉"));
        assert_eq!(both.to_string(), "🎉 Party");

        assert!(PollMedia::default().is_empty());
        assert_eq!(PollMedia::default().to_string(), "");
        assert_eq!(PollMedia::from("text"), PollMedia::new("text"));
        assert_eq!(PollMedia::from("text".to_owned()), PollMedia::new("text"));
    }

    #[test]
    fn poll_answers_and_display() {
        let answer = PollAnswer::new(1, "Yes")
            .with_vote_count(4)
            .with_self_voted(true);

        assert_eq!(answer.answer_id, 1);
        assert_eq!(answer.text(), Some("Yes"));
        assert_eq!(answer.emoji(), None);
        assert_eq!(answer.vote_count, 4);
        assert!(answer.self_voted);
        assert_eq!(answer.to_string(), "Yes");

        let emoji_answer = PollAnswer::new(2, PollMedia::from_emoji(PartialEmoji::unicode("❌")));
        assert_eq!(emoji_answer.text(), None);
        assert_eq!(
            emoji_answer.emoji().map(ToString::to_string),
            Some("❌".to_owned())
        );
        assert_eq!(emoji_answer.to_string(), "❌");
    }

    #[test]
    fn poll_results_and_counts() {
        let results = PollResults::new()
            .with_answer_counts(vec![
                PollAnswerCount::new(1, 3),
                PollAnswerCount::new(2, 5).with_me_voted(true),
            ])
            .add_count(PollAnswerCount::new(3, 1))
            .with_is_finalized(true);

        assert!(results.is_finalized);
        assert_eq!(results.answer_counts.len(), 3);
        assert_eq!(results.total_votes(), 9);
        assert_eq!(results.get_count(2).map(|count| count.count), Some(5));
        assert!(results.get_count(2).is_some_and(|count| count.me_voted));
        assert!(results.get_count(9).is_none());
    }

    #[test]
    fn poll_builders_and_summaries() {
        let expiry = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let poll = Poll::new("Favourite?")
            .with_expiry(expiry)
            .with_allow_multiselect(true)
            .with_layout_type(PollLayoutType::Default)
            .add_answer(PollAnswer::new(1, "Yes").with_vote_count(2))
            .add_answer(PollAnswer::new(2, "No").with_vote_count(1));

        assert_eq!(poll.question_text(), Some("Favourite?"));
        assert_eq!(poll.answers().len(), 2);
        assert_eq!(poll.get_answer(1).map(|answer| answer.vote_count), Some(2));
        assert!(poll.get_answer(3).is_none());
        assert_eq!(poll.expires_at(), Some(expiry));
        assert!(poll.allow_multiselect);
        assert_eq!(poll.layout_type, PollLayoutType::Default);
        assert!(!poll.is_finalized());
        assert!(!poll.is_finalised());
        assert_eq!(poll.total_votes(), 3);
        assert_eq!(poll.victor_answer_id(), None);
        assert_eq!(poll.victor_answer(), None);
    }

    #[test]
    fn poll_victor_uses_finalized_results() {
        let counts = vec![
            PollAnswerCount::new(1, 2),
            PollAnswerCount::new(2, 5),
            PollAnswerCount::new(3, 1),
        ];

        let pending = Poll::new("Favourite?")
            .with_answers(vec![
                PollAnswer::new(1, "A"),
                PollAnswer::new(2, "B"),
                PollAnswer::new(3, "C"),
            ])
            .with_results(PollResults::new().with_answer_counts(counts.clone()));

        assert_eq!(pending.victor_answer_id(), None);
        assert_eq!(pending.total_votes(), 8);

        let finalized = pending.with_results(
            PollResults::new()
                .with_answer_counts(counts)
                .with_is_finalized(true),
        );

        assert!(finalized.is_finalized());
        assert_eq!(finalized.victor_answer_id(), Some(2));
        assert_eq!(
            finalized.victor_answer().map(|answer| answer.text()),
            Some(Some("B"))
        );
        assert_eq!(finalized.total_votes(), 8);
    }

    #[test]
    fn decodes_poll_objects() {
        let value = json::from_str(
            r#"{
                "question": {"text": "Favourite?"},
                "answers": [
                    {
                        "answer_id": 1,
                        "poll_media": {"text": "A", "emoji": {"name": "🅰️"}}
                    },
                    {
                        "answer_id": 2,
                        "poll_media": {"text": "B"}
                    }
                ],
                "expiry": "2021-08-01T12:34:56Z",
                "allow_multiselect": true,
                "layout_type": 1,
                "results": {
                    "is_finalized": true,
                    "answer_counts": [
                        {"id": 1, "count": 3, "me_voted": true},
                        {"id": 2, "count": 5}
                    ]
                }
            }"#,
        )
        .unwrap();

        let poll = Poll::from_value(&value).unwrap();
        assert_eq!(poll.question_text(), Some("Favourite?"));
        assert_eq!(poll.answers().len(), 2);
        assert_eq!(poll.answers()[0].text(), Some("A"));
        assert_eq!(
            poll.answers()[0]
                .emoji()
                .and_then(|emoji| emoji.name.as_deref()),
            Some("🅰️")
        );
        assert_eq!(poll.answers()[1].answer_id, 2);
        assert_eq!(
            poll.expiry,
            Some(UNIX_EPOCH + Duration::new(1_627_821_296, 0))
        );
        assert!(poll.allow_multiselect);
        assert_eq!(poll.layout_type, PollLayoutType::Default);
        assert!(poll.is_finalized());
        assert_eq!(poll.total_votes(), 8);
        assert_eq!(poll.victor_answer_id(), Some(2));

        let minimal =
            Poll::from_value(&json::from_str(r#"{"question":{"text":"Q"}}"#).unwrap()).unwrap();
        assert_eq!(minimal.answers().len(), 0);
        assert_eq!(minimal.expiry, None);
        assert!(!minimal.allow_multiselect);
        assert_eq!(minimal.results, None);

        let missing_question =
            Poll::from_value(&json::from_str(r#"{"answers":[]}"#).unwrap()).unwrap_err();
        assert_eq!(
            missing_question.to_string(),
            "missing field `poll.question`"
        );

        let bad_answer = Poll::from_value(
            &json::from_str(r#"{"question":{"text":"Q"},"answers":[{"poll_media":{"text":"A"}}]}"#)
                .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_answer.to_string(),
            "missing field `poll.answers[0].answer_id`"
        );

        let bad_results = Poll::from_value(
            &json::from_str(r#"{"question":{"text":"Q"},"results":{"answer_counts":[]}}"#).unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            bad_results.to_string(),
            "missing field `poll.results.is_finalized`"
        );
    }
}
