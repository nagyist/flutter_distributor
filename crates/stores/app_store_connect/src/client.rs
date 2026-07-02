#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///`AgeRatingDeclaration`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AgeRatingDeclaration",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "advertising": {
    ///          "type": "boolean"
    ///        },
    ///        "ageAssurance": {
    ///          "type": "boolean"
    ///        },
    ///        "ageRatingOverride": {
    ///          "deprecated": true,
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "NINE_PLUS",
    ///            "THIRTEEN_PLUS",
    ///            "SIXTEEN_PLUS",
    ///            "SEVENTEEN_PLUS",
    ///            "UNRATED"
    ///          ]
    ///        },
    ///        "ageRatingOverrideV2": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "NINE_PLUS",
    ///            "THIRTEEN_PLUS",
    ///            "SIXTEEN_PLUS",
    ///            "EIGHTEEN_PLUS",
    ///            "UNRATED"
    ///          ]
    ///        },
    ///        "alcoholTobaccoOrDrugUseOrReferences": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        },
    ///        "contests": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        },
    ///        "developerAgeRatingInfoUrl": {
    ///          "type": "string",
    ///          "format": "uri"
    ///        },
    ///        "gambling": {
    ///          "type": "boolean"
    ///        },
    ///        "gamblingSimulated": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        },
    ///        "gunsOrOtherWeapons": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        },
    ///        "healthOrWellnessTopics": {
    ///          "type": "boolean"
    ///        },
    ///        "horrorOrFearThemes": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        },
    ///        "kidsAgeBand": {
    ///          "$ref": "#/components/schemas/KidsAgeBand"
    ///        },
    ///        "koreaAgeRatingOverride": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "FIFTEEN_PLUS",
    ///            "NINETEEN_PLUS"
    ///          ]
    ///        },
    ///        "lootBox": {
    ///          "type": "boolean"
    ///        },
    ///        "matureOrSuggestiveThemes": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        },
    ///        "medicalOrTreatmentInformation": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        },
    ///        "messagingAndChat": {
    ///          "type": "boolean"
    ///        },
    ///        "parentalControls": {
    ///          "type": "boolean"
    ///        },
    ///        "profanityOrCrudeHumor": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        },
    ///        "sexualContentGraphicAndNudity": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        },
    ///        "sexualContentOrNudity": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        },
    ///        "unrestrictedWebAccess": {
    ///          "type": "boolean"
    ///        },
    ///        "userGeneratedContent": {
    ///          "type": "boolean"
    ///        },
    ///        "violenceCartoonOrFantasy": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        },
    ///        "violenceRealistic": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        },
    ///        "violenceRealisticProlongedGraphicOrSadistic": {
    ///          "type": "string",
    ///          "enum": [
    ///            "NONE",
    ///            "INFREQUENT_OR_MILD",
    ///            "FREQUENT_OR_INTENSE",
    ///            "INFREQUENT",
    ///            "FREQUENT"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "ageRatingDeclarations"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AgeRatingDeclaration {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AgeRatingDeclarationAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(rename = "type")]
        pub type_: AgeRatingDeclarationType,
    }

    ///`AgeRatingDeclarationAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "advertising": {
    ///      "type": "boolean"
    ///    },
    ///    "ageAssurance": {
    ///      "type": "boolean"
    ///    },
    ///    "ageRatingOverride": {
    ///      "deprecated": true,
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "NINE_PLUS",
    ///        "THIRTEEN_PLUS",
    ///        "SIXTEEN_PLUS",
    ///        "SEVENTEEN_PLUS",
    ///        "UNRATED"
    ///      ]
    ///    },
    ///    "ageRatingOverrideV2": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "NINE_PLUS",
    ///        "THIRTEEN_PLUS",
    ///        "SIXTEEN_PLUS",
    ///        "EIGHTEEN_PLUS",
    ///        "UNRATED"
    ///      ]
    ///    },
    ///    "alcoholTobaccoOrDrugUseOrReferences": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    },
    ///    "contests": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    },
    ///    "developerAgeRatingInfoUrl": {
    ///      "type": "string",
    ///      "format": "uri"
    ///    },
    ///    "gambling": {
    ///      "type": "boolean"
    ///    },
    ///    "gamblingSimulated": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    },
    ///    "gunsOrOtherWeapons": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    },
    ///    "healthOrWellnessTopics": {
    ///      "type": "boolean"
    ///    },
    ///    "horrorOrFearThemes": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    },
    ///    "kidsAgeBand": {
    ///      "$ref": "#/components/schemas/KidsAgeBand"
    ///    },
    ///    "koreaAgeRatingOverride": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "FIFTEEN_PLUS",
    ///        "NINETEEN_PLUS"
    ///      ]
    ///    },
    ///    "lootBox": {
    ///      "type": "boolean"
    ///    },
    ///    "matureOrSuggestiveThemes": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    },
    ///    "medicalOrTreatmentInformation": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    },
    ///    "messagingAndChat": {
    ///      "type": "boolean"
    ///    },
    ///    "parentalControls": {
    ///      "type": "boolean"
    ///    },
    ///    "profanityOrCrudeHumor": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    },
    ///    "sexualContentGraphicAndNudity": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    },
    ///    "sexualContentOrNudity": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    },
    ///    "unrestrictedWebAccess": {
    ///      "type": "boolean"
    ///    },
    ///    "userGeneratedContent": {
    ///      "type": "boolean"
    ///    },
    ///    "violenceCartoonOrFantasy": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    },
    ///    "violenceRealistic": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    },
    ///    "violenceRealisticProlongedGraphicOrSadistic": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "INFREQUENT_OR_MILD",
    ///        "FREQUENT_OR_INTENSE",
    ///        "INFREQUENT",
    ///        "FREQUENT"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AgeRatingDeclarationAttributes {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub advertising: ::std::option::Option<bool>,
        #[serde(
            rename = "ageAssurance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub age_assurance: ::std::option::Option<bool>,
        #[serde(
            rename = "ageRatingOverride",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub age_rating_override:
            ::std::option::Option<AgeRatingDeclarationAttributesAgeRatingOverride>,
        #[serde(
            rename = "ageRatingOverrideV2",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub age_rating_override_v2:
            ::std::option::Option<AgeRatingDeclarationAttributesAgeRatingOverrideV2>,
        #[serde(
            rename = "alcoholTobaccoOrDrugUseOrReferences",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub alcohol_tobacco_or_drug_use_or_references: ::std::option::Option<
            AgeRatingDeclarationAttributesAlcoholTobaccoOrDrugUseOrReferences,
        >,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub contests: ::std::option::Option<AgeRatingDeclarationAttributesContests>,
        #[serde(
            rename = "developerAgeRatingInfoUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub developer_age_rating_info_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gambling: ::std::option::Option<bool>,
        #[serde(
            rename = "gamblingSimulated",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub gambling_simulated:
            ::std::option::Option<AgeRatingDeclarationAttributesGamblingSimulated>,
        #[serde(
            rename = "gunsOrOtherWeapons",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub guns_or_other_weapons:
            ::std::option::Option<AgeRatingDeclarationAttributesGunsOrOtherWeapons>,
        #[serde(
            rename = "healthOrWellnessTopics",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub health_or_wellness_topics: ::std::option::Option<bool>,
        #[serde(
            rename = "horrorOrFearThemes",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub horror_or_fear_themes:
            ::std::option::Option<AgeRatingDeclarationAttributesHorrorOrFearThemes>,
        #[serde(
            rename = "kidsAgeBand",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub kids_age_band: ::std::option::Option<KidsAgeBand>,
        #[serde(
            rename = "koreaAgeRatingOverride",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub korea_age_rating_override:
            ::std::option::Option<AgeRatingDeclarationAttributesKoreaAgeRatingOverride>,
        #[serde(
            rename = "lootBox",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub loot_box: ::std::option::Option<bool>,
        #[serde(
            rename = "matureOrSuggestiveThemes",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mature_or_suggestive_themes:
            ::std::option::Option<AgeRatingDeclarationAttributesMatureOrSuggestiveThemes>,
        #[serde(
            rename = "medicalOrTreatmentInformation",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub medical_or_treatment_information:
            ::std::option::Option<AgeRatingDeclarationAttributesMedicalOrTreatmentInformation>,
        #[serde(
            rename = "messagingAndChat",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub messaging_and_chat: ::std::option::Option<bool>,
        #[serde(
            rename = "parentalControls",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub parental_controls: ::std::option::Option<bool>,
        #[serde(
            rename = "profanityOrCrudeHumor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub profanity_or_crude_humor:
            ::std::option::Option<AgeRatingDeclarationAttributesProfanityOrCrudeHumor>,
        #[serde(
            rename = "sexualContentGraphicAndNudity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sexual_content_graphic_and_nudity:
            ::std::option::Option<AgeRatingDeclarationAttributesSexualContentGraphicAndNudity>,
        #[serde(
            rename = "sexualContentOrNudity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sexual_content_or_nudity:
            ::std::option::Option<AgeRatingDeclarationAttributesSexualContentOrNudity>,
        #[serde(
            rename = "unrestrictedWebAccess",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub unrestricted_web_access: ::std::option::Option<bool>,
        #[serde(
            rename = "userGeneratedContent",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_generated_content: ::std::option::Option<bool>,
        #[serde(
            rename = "violenceCartoonOrFantasy",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub violence_cartoon_or_fantasy:
            ::std::option::Option<AgeRatingDeclarationAttributesViolenceCartoonOrFantasy>,
        #[serde(
            rename = "violenceRealistic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub violence_realistic:
            ::std::option::Option<AgeRatingDeclarationAttributesViolenceRealistic>,
        #[serde(
            rename = "violenceRealisticProlongedGraphicOrSadistic",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub violence_realistic_prolonged_graphic_or_sadistic: ::std::option::Option<
            AgeRatingDeclarationAttributesViolenceRealisticProlongedGraphicOrSadistic,
        >,
    }

    impl ::std::default::Default for AgeRatingDeclarationAttributes {
        fn default() -> Self {
            Self {
                advertising: Default::default(),
                age_assurance: Default::default(),
                age_rating_override: Default::default(),
                age_rating_override_v2: Default::default(),
                alcohol_tobacco_or_drug_use_or_references: Default::default(),
                contests: Default::default(),
                developer_age_rating_info_url: Default::default(),
                gambling: Default::default(),
                gambling_simulated: Default::default(),
                guns_or_other_weapons: Default::default(),
                health_or_wellness_topics: Default::default(),
                horror_or_fear_themes: Default::default(),
                kids_age_band: Default::default(),
                korea_age_rating_override: Default::default(),
                loot_box: Default::default(),
                mature_or_suggestive_themes: Default::default(),
                medical_or_treatment_information: Default::default(),
                messaging_and_chat: Default::default(),
                parental_controls: Default::default(),
                profanity_or_crude_humor: Default::default(),
                sexual_content_graphic_and_nudity: Default::default(),
                sexual_content_or_nudity: Default::default(),
                unrestricted_web_access: Default::default(),
                user_generated_content: Default::default(),
                violence_cartoon_or_fantasy: Default::default(),
                violence_realistic: Default::default(),
                violence_realistic_prolonged_graphic_or_sadistic: Default::default(),
            }
        }
    }

    ///`AgeRatingDeclarationAttributesAgeRatingOverride`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "deprecated": true,
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "NINE_PLUS",
    ///    "THIRTEEN_PLUS",
    ///    "SIXTEEN_PLUS",
    ///    "SEVENTEEN_PLUS",
    ///    "UNRATED"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesAgeRatingOverride {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "NINE_PLUS")]
        NinePlus,
        #[serde(rename = "THIRTEEN_PLUS")]
        ThirteenPlus,
        #[serde(rename = "SIXTEEN_PLUS")]
        SixteenPlus,
        #[serde(rename = "SEVENTEEN_PLUS")]
        SeventeenPlus,
        #[serde(rename = "UNRATED")]
        Unrated,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesAgeRatingOverride {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::NinePlus => f.write_str("NINE_PLUS"),
                Self::ThirteenPlus => f.write_str("THIRTEEN_PLUS"),
                Self::SixteenPlus => f.write_str("SIXTEEN_PLUS"),
                Self::SeventeenPlus => f.write_str("SEVENTEEN_PLUS"),
                Self::Unrated => f.write_str("UNRATED"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesAgeRatingOverride {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "NINE_PLUS" => Ok(Self::NinePlus),
                "THIRTEEN_PLUS" => Ok(Self::ThirteenPlus),
                "SIXTEEN_PLUS" => Ok(Self::SixteenPlus),
                "SEVENTEEN_PLUS" => Ok(Self::SeventeenPlus),
                "UNRATED" => Ok(Self::Unrated),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesAgeRatingOverride {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesAgeRatingOverride
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesAgeRatingOverride
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesAgeRatingOverrideV2`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "NINE_PLUS",
    ///    "THIRTEEN_PLUS",
    ///    "SIXTEEN_PLUS",
    ///    "EIGHTEEN_PLUS",
    ///    "UNRATED"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesAgeRatingOverrideV2 {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "NINE_PLUS")]
        NinePlus,
        #[serde(rename = "THIRTEEN_PLUS")]
        ThirteenPlus,
        #[serde(rename = "SIXTEEN_PLUS")]
        SixteenPlus,
        #[serde(rename = "EIGHTEEN_PLUS")]
        EighteenPlus,
        #[serde(rename = "UNRATED")]
        Unrated,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesAgeRatingOverrideV2 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::NinePlus => f.write_str("NINE_PLUS"),
                Self::ThirteenPlus => f.write_str("THIRTEEN_PLUS"),
                Self::SixteenPlus => f.write_str("SIXTEEN_PLUS"),
                Self::EighteenPlus => f.write_str("EIGHTEEN_PLUS"),
                Self::Unrated => f.write_str("UNRATED"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesAgeRatingOverrideV2 {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "NINE_PLUS" => Ok(Self::NinePlus),
                "THIRTEEN_PLUS" => Ok(Self::ThirteenPlus),
                "SIXTEEN_PLUS" => Ok(Self::SixteenPlus),
                "EIGHTEEN_PLUS" => Ok(Self::EighteenPlus),
                "UNRATED" => Ok(Self::Unrated),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesAgeRatingOverrideV2 {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesAgeRatingOverrideV2
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesAgeRatingOverrideV2
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesAlcoholTobaccoOrDrugUseOrReferences`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesAlcoholTobaccoOrDrugUseOrReferences {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesAlcoholTobaccoOrDrugUseOrReferences {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesAlcoholTobaccoOrDrugUseOrReferences {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AgeRatingDeclarationAttributesAlcoholTobaccoOrDrugUseOrReferences
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesAlcoholTobaccoOrDrugUseOrReferences
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesAlcoholTobaccoOrDrugUseOrReferences
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesContests`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesContests {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesContests {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesContests {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesContests {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AgeRatingDeclarationAttributesContests {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AgeRatingDeclarationAttributesContests {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesGamblingSimulated`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesGamblingSimulated {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesGamblingSimulated {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesGamblingSimulated {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesGamblingSimulated {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesGamblingSimulated
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesGamblingSimulated
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesGunsOrOtherWeapons`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesGunsOrOtherWeapons {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesGunsOrOtherWeapons {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesGunsOrOtherWeapons {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesGunsOrOtherWeapons {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesGunsOrOtherWeapons
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesGunsOrOtherWeapons
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesHorrorOrFearThemes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesHorrorOrFearThemes {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesHorrorOrFearThemes {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesHorrorOrFearThemes {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesHorrorOrFearThemes {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesHorrorOrFearThemes
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesHorrorOrFearThemes
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesKoreaAgeRatingOverride`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "FIFTEEN_PLUS",
    ///    "NINETEEN_PLUS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesKoreaAgeRatingOverride {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "FIFTEEN_PLUS")]
        FifteenPlus,
        #[serde(rename = "NINETEEN_PLUS")]
        NineteenPlus,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesKoreaAgeRatingOverride {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::FifteenPlus => f.write_str("FIFTEEN_PLUS"),
                Self::NineteenPlus => f.write_str("NINETEEN_PLUS"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesKoreaAgeRatingOverride {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "FIFTEEN_PLUS" => Ok(Self::FifteenPlus),
                "NINETEEN_PLUS" => Ok(Self::NineteenPlus),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesKoreaAgeRatingOverride {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesKoreaAgeRatingOverride
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesKoreaAgeRatingOverride
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesMatureOrSuggestiveThemes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesMatureOrSuggestiveThemes {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesMatureOrSuggestiveThemes {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesMatureOrSuggestiveThemes {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesMatureOrSuggestiveThemes {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesMatureOrSuggestiveThemes
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesMatureOrSuggestiveThemes
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesMedicalOrTreatmentInformation`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesMedicalOrTreatmentInformation {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesMedicalOrTreatmentInformation {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesMedicalOrTreatmentInformation {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesMedicalOrTreatmentInformation {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesMedicalOrTreatmentInformation
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesMedicalOrTreatmentInformation
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesProfanityOrCrudeHumor`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesProfanityOrCrudeHumor {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesProfanityOrCrudeHumor {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesProfanityOrCrudeHumor {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesProfanityOrCrudeHumor {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesProfanityOrCrudeHumor
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesProfanityOrCrudeHumor
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesSexualContentGraphicAndNudity`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesSexualContentGraphicAndNudity {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesSexualContentGraphicAndNudity {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesSexualContentGraphicAndNudity {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesSexualContentGraphicAndNudity {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesSexualContentGraphicAndNudity
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesSexualContentGraphicAndNudity
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesSexualContentOrNudity`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesSexualContentOrNudity {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesSexualContentOrNudity {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesSexualContentOrNudity {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesSexualContentOrNudity {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesSexualContentOrNudity
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesSexualContentOrNudity
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesViolenceCartoonOrFantasy`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesViolenceCartoonOrFantasy {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesViolenceCartoonOrFantasy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesViolenceCartoonOrFantasy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesViolenceCartoonOrFantasy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesViolenceCartoonOrFantasy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesViolenceCartoonOrFantasy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesViolenceRealistic`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesViolenceRealistic {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationAttributesViolenceRealistic {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationAttributesViolenceRealistic {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationAttributesViolenceRealistic {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesViolenceRealistic
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesViolenceRealistic
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationAttributesViolenceRealisticProlongedGraphicOrSadistic`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "INFREQUENT_OR_MILD",
    ///    "FREQUENT_OR_INTENSE",
    ///    "INFREQUENT",
    ///    "FREQUENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationAttributesViolenceRealisticProlongedGraphicOrSadistic {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "INFREQUENT_OR_MILD")]
        InfrequentOrMild,
        #[serde(rename = "FREQUENT_OR_INTENSE")]
        FrequentOrIntense,
        #[serde(rename = "INFREQUENT")]
        Infrequent,
        #[serde(rename = "FREQUENT")]
        Frequent,
    }

    impl ::std::fmt::Display
        for AgeRatingDeclarationAttributesViolenceRealisticProlongedGraphicOrSadistic
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::InfrequentOrMild => f.write_str("INFREQUENT_OR_MILD"),
                Self::FrequentOrIntense => f.write_str("FREQUENT_OR_INTENSE"),
                Self::Infrequent => f.write_str("INFREQUENT"),
                Self::Frequent => f.write_str("FREQUENT"),
            }
        }
    }

    impl ::std::str::FromStr
        for AgeRatingDeclarationAttributesViolenceRealisticProlongedGraphicOrSadistic
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INFREQUENT_OR_MILD" => Ok(Self::InfrequentOrMild),
                "FREQUENT_OR_INTENSE" => Ok(Self::FrequentOrIntense),
                "INFREQUENT" => Ok(Self::Infrequent),
                "FREQUENT" => Ok(Self::Frequent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AgeRatingDeclarationAttributesViolenceRealisticProlongedGraphicOrSadistic
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AgeRatingDeclarationAttributesViolenceRealisticProlongedGraphicOrSadistic
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AgeRatingDeclarationAttributesViolenceRealisticProlongedGraphicOrSadistic
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AgeRatingDeclarationType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ageRatingDeclarations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AgeRatingDeclarationType {
        #[serde(rename = "ageRatingDeclarations")]
        AgeRatingDeclarations,
    }

    impl ::std::fmt::Display for AgeRatingDeclarationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AgeRatingDeclarations => f.write_str("ageRatingDeclarations"),
            }
        }
    }

    impl ::std::str::FromStr for AgeRatingDeclarationType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ageRatingDeclarations" => Ok(Self::AgeRatingDeclarations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AgeRatingDeclarationType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AgeRatingDeclarationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AgeRatingDeclarationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`App`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "App",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "bundleId": {
    ///          "type": "string"
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "apps"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct App {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(rename = "type")]
        pub type_: AppType,
    }

    ///`AppAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "bundleId": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppAttributes {
        #[serde(
            rename = "bundleId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bundle_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for AppAttributes {
        fn default() -> Self {
            Self {
                bundle_id: Default::default(),
                name: Default::default(),
            }
        }
    }

    ///`AppCategory`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppCategory",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "platforms": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/Platform"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "relationships": {
    ///      "type": "object",
    ///      "properties": {
    ///        "parent": {
    ///          "type": "object",
    ///          "properties": {
    ///            "data": {
    ///              "type": "object",
    ///              "required": [
    ///                "id",
    ///                "type"
    ///              ],
    ///              "properties": {
    ///                "id": {
    ///                  "type": "string"
    ///                },
    ///                "type": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "appCategories"
    ///                  ]
    ///                }
    ///              }
    ///            },
    ///            "links": {
    ///              "$ref": "#/components/schemas/RelationshipLinks"
    ///            }
    ///          }
    ///        },
    ///        "subcategories": {
    ///          "type": "object",
    ///          "properties": {
    ///            "data": {
    ///              "type": "array",
    ///              "items": {
    ///                "type": "object",
    ///                "required": [
    ///                  "id",
    ///                  "type"
    ///                ],
    ///                "properties": {
    ///                  "id": {
    ///                    "type": "string"
    ///                  },
    ///                  "type": {
    ///                    "type": "string",
    ///                    "enum": [
    ///                      "appCategories"
    ///                    ]
    ///                  }
    ///                }
    ///              }
    ///            },
    ///            "links": {
    ///              "$ref": "#/components/schemas/RelationshipLinks"
    ///            },
    ///            "meta": {
    ///              "$ref": "#/components/schemas/PagingInformation"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appCategories"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppCategory {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppCategoryAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub relationships: ::std::option::Option<AppCategoryRelationships>,
        #[serde(rename = "type")]
        pub type_: AppCategoryType,
    }

    ///`AppCategoryAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "platforms": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Platform"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppCategoryAttributes {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub platforms: ::std::vec::Vec<Platform>,
    }

    impl ::std::default::Default for AppCategoryAttributes {
        fn default() -> Self {
            Self {
                platforms: Default::default(),
            }
        }
    }

    ///`AppCategoryRelationships`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "parent": {
    ///      "type": "object",
    ///      "properties": {
    ///        "data": {
    ///          "type": "object",
    ///          "required": [
    ///            "id",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "id": {
    ///              "type": "string"
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "appCategories"
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        "links": {
    ///          "$ref": "#/components/schemas/RelationshipLinks"
    ///        }
    ///      }
    ///    },
    ///    "subcategories": {
    ///      "type": "object",
    ///      "properties": {
    ///        "data": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "object",
    ///            "required": [
    ///              "id",
    ///              "type"
    ///            ],
    ///            "properties": {
    ///              "id": {
    ///                "type": "string"
    ///              },
    ///              "type": {
    ///                "type": "string",
    ///                "enum": [
    ///                  "appCategories"
    ///                ]
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "links": {
    ///          "$ref": "#/components/schemas/RelationshipLinks"
    ///        },
    ///        "meta": {
    ///          "$ref": "#/components/schemas/PagingInformation"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppCategoryRelationships {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub parent: ::std::option::Option<AppCategoryRelationshipsParent>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub subcategories: ::std::option::Option<AppCategoryRelationshipsSubcategories>,
    }

    impl ::std::default::Default for AppCategoryRelationships {
        fn default() -> Self {
            Self {
                parent: Default::default(),
                subcategories: Default::default(),
            }
        }
    }

    ///`AppCategoryRelationshipsParent`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "appCategories"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/RelationshipLinks"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppCategoryRelationshipsParent {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<AppCategoryRelationshipsParentData>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<RelationshipLinks>,
    }

    impl ::std::default::Default for AppCategoryRelationshipsParent {
        fn default() -> Self {
            Self {
                data: Default::default(),
                links: Default::default(),
            }
        }
    }

    ///`AppCategoryRelationshipsParentData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appCategories"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppCategoryRelationshipsParentData {
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppCategoryRelationshipsParentDataType,
    }

    ///`AppCategoryRelationshipsParentDataType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appCategories"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppCategoryRelationshipsParentDataType {
        #[serde(rename = "appCategories")]
        AppCategories,
    }

    impl ::std::fmt::Display for AppCategoryRelationshipsParentDataType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppCategories => f.write_str("appCategories"),
            }
        }
    }

    impl ::std::str::FromStr for AppCategoryRelationshipsParentDataType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appCategories" => Ok(Self::AppCategories),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppCategoryRelationshipsParentDataType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppCategoryRelationshipsParentDataType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppCategoryRelationshipsParentDataType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppCategoryRelationshipsSubcategories`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "id",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "id": {
    ///            "type": "string"
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "appCategories"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/RelationshipLinks"
    ///    },
    ///    "meta": {
    ///      "$ref": "#/components/schemas/PagingInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppCategoryRelationshipsSubcategories {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub data: ::std::vec::Vec<AppCategoryRelationshipsSubcategoriesDataItem>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<RelationshipLinks>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub meta: ::std::option::Option<PagingInformation>,
    }

    impl ::std::default::Default for AppCategoryRelationshipsSubcategories {
        fn default() -> Self {
            Self {
                data: Default::default(),
                links: Default::default(),
                meta: Default::default(),
            }
        }
    }

    ///`AppCategoryRelationshipsSubcategoriesDataItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appCategories"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppCategoryRelationshipsSubcategoriesDataItem {
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppCategoryRelationshipsSubcategoriesDataItemType,
    }

    ///`AppCategoryRelationshipsSubcategoriesDataItemType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appCategories"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppCategoryRelationshipsSubcategoriesDataItemType {
        #[serde(rename = "appCategories")]
        AppCategories,
    }

    impl ::std::fmt::Display for AppCategoryRelationshipsSubcategoriesDataItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppCategories => f.write_str("appCategories"),
            }
        }
    }

    impl ::std::str::FromStr for AppCategoryRelationshipsSubcategoriesDataItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appCategories" => Ok(Self::AppCategories),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppCategoryRelationshipsSubcategoriesDataItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppCategoryRelationshipsSubcategoriesDataItemType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppCategoryRelationshipsSubcategoriesDataItemType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppCategoryType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appCategories"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppCategoryType {
        #[serde(rename = "appCategories")]
        AppCategories,
    }

    impl ::std::fmt::Display for AppCategoryType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppCategories => f.write_str("appCategories"),
            }
        }
    }

    impl ::std::str::FromStr for AppCategoryType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appCategories" => Ok(Self::AppCategories),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppCategoryType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppCategoryType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppCategoryType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppInfo",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "appStoreAgeRating": {
    ///          "$ref": "#/components/schemas/AppStoreAgeRating"
    ///        },
    ///        "appStoreState": {
    ///          "$ref": "#/components/schemas/AppStoreVersionState"
    ///        },
    ///        "australiaAgeRating": {
    ///          "deprecated": true,
    ///          "type": "string",
    ///          "enum": [
    ///            "FIFTEEN",
    ///            "EIGHTEEN"
    ///          ]
    ///        },
    ///        "brazilAgeRating": {
    ///          "$ref": "#/components/schemas/BrazilAgeRating"
    ///        },
    ///        "brazilAgeRatingV2": {
    ///          "deprecated": true,
    ///          "type": "string",
    ///          "enum": [
    ///            "SELF_RATED_L",
    ///            "SELF_RATED_TEN",
    ///            "SELF_RATED_TWELVE",
    ///            "SELF_RATED_FOURTEEN",
    ///            "SELF_RATED_SIXTEEN",
    ///            "SELF_RATED_EIGHTEEN",
    ///            "OFFICIAL_L",
    ///            "OFFICIAL_TEN",
    ///            "OFFICIAL_TWELVE",
    ///            "OFFICIAL_FOURTEEN",
    ///            "OFFICIAL_SIXTEEN",
    ///            "OFFICIAL_EIGHTEEN"
    ///          ]
    ///        },
    ///        "franceAgeRating": {
    ///          "deprecated": true,
    ///          "type": "string",
    ///          "enum": [
    ///            "EIGHTEEN"
    ///          ]
    ///        },
    ///        "koreaAgeRating": {
    ///          "deprecated": true,
    ///          "type": "string",
    ///          "enum": [
    ///            "ALL",
    ///            "TWELVE",
    ///            "FIFTEEN",
    ///            "NINETEEN",
    ///            "NOT_APPLICABLE"
    ///          ]
    ///        },
    ///        "state": {
    ///          "type": "string",
    ///          "enum": [
    ///            "ACCEPTED",
    ///            "DEVELOPER_REJECTED",
    ///            "IN_REVIEW",
    ///            "PENDING_RELEASE",
    ///            "PREPARE_FOR_SUBMISSION",
    ///            "READY_FOR_DISTRIBUTION",
    ///            "READY_FOR_REVIEW",
    ///            "REJECTED",
    ///            "REPLACED_WITH_NEW_INFO",
    ///            "WAITING_FOR_REVIEW"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appInfos"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfo {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppInfoAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(rename = "type")]
        pub type_: AppInfoType,
    }

    ///`AppInfoAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "appStoreAgeRating": {
    ///      "$ref": "#/components/schemas/AppStoreAgeRating"
    ///    },
    ///    "appStoreState": {
    ///      "$ref": "#/components/schemas/AppStoreVersionState"
    ///    },
    ///    "australiaAgeRating": {
    ///      "deprecated": true,
    ///      "type": "string",
    ///      "enum": [
    ///        "FIFTEEN",
    ///        "EIGHTEEN"
    ///      ]
    ///    },
    ///    "brazilAgeRating": {
    ///      "$ref": "#/components/schemas/BrazilAgeRating"
    ///    },
    ///    "brazilAgeRatingV2": {
    ///      "deprecated": true,
    ///      "type": "string",
    ///      "enum": [
    ///        "SELF_RATED_L",
    ///        "SELF_RATED_TEN",
    ///        "SELF_RATED_TWELVE",
    ///        "SELF_RATED_FOURTEEN",
    ///        "SELF_RATED_SIXTEEN",
    ///        "SELF_RATED_EIGHTEEN",
    ///        "OFFICIAL_L",
    ///        "OFFICIAL_TEN",
    ///        "OFFICIAL_TWELVE",
    ///        "OFFICIAL_FOURTEEN",
    ///        "OFFICIAL_SIXTEEN",
    ///        "OFFICIAL_EIGHTEEN"
    ///      ]
    ///    },
    ///    "franceAgeRating": {
    ///      "deprecated": true,
    ///      "type": "string",
    ///      "enum": [
    ///        "EIGHTEEN"
    ///      ]
    ///    },
    ///    "koreaAgeRating": {
    ///      "deprecated": true,
    ///      "type": "string",
    ///      "enum": [
    ///        "ALL",
    ///        "TWELVE",
    ///        "FIFTEEN",
    ///        "NINETEEN",
    ///        "NOT_APPLICABLE"
    ///      ]
    ///    },
    ///    "state": {
    ///      "type": "string",
    ///      "enum": [
    ///        "ACCEPTED",
    ///        "DEVELOPER_REJECTED",
    ///        "IN_REVIEW",
    ///        "PENDING_RELEASE",
    ///        "PREPARE_FOR_SUBMISSION",
    ///        "READY_FOR_DISTRIBUTION",
    ///        "READY_FOR_REVIEW",
    ///        "REJECTED",
    ///        "REPLACED_WITH_NEW_INFO",
    ///        "WAITING_FOR_REVIEW"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfoAttributes {
        #[serde(
            rename = "appStoreAgeRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_store_age_rating: ::std::option::Option<AppStoreAgeRating>,
        #[serde(
            rename = "appStoreState",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_store_state: ::std::option::Option<AppStoreVersionState>,
        #[serde(
            rename = "australiaAgeRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub australia_age_rating: ::std::option::Option<AppInfoAttributesAustraliaAgeRating>,
        #[serde(
            rename = "brazilAgeRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub brazil_age_rating: ::std::option::Option<BrazilAgeRating>,
        #[serde(
            rename = "brazilAgeRatingV2",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub brazil_age_rating_v2: ::std::option::Option<AppInfoAttributesBrazilAgeRatingV2>,
        #[serde(
            rename = "franceAgeRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub france_age_rating: ::std::option::Option<AppInfoAttributesFranceAgeRating>,
        #[serde(
            rename = "koreaAgeRating",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub korea_age_rating: ::std::option::Option<AppInfoAttributesKoreaAgeRating>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<AppInfoAttributesState>,
    }

    impl ::std::default::Default for AppInfoAttributes {
        fn default() -> Self {
            Self {
                app_store_age_rating: Default::default(),
                app_store_state: Default::default(),
                australia_age_rating: Default::default(),
                brazil_age_rating: Default::default(),
                brazil_age_rating_v2: Default::default(),
                france_age_rating: Default::default(),
                korea_age_rating: Default::default(),
                state: Default::default(),
            }
        }
    }

    ///`AppInfoAttributesAustraliaAgeRating`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "deprecated": true,
    ///  "type": "string",
    ///  "enum": [
    ///    "FIFTEEN",
    ///    "EIGHTEEN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfoAttributesAustraliaAgeRating {
        #[serde(rename = "FIFTEEN")]
        Fifteen,
        #[serde(rename = "EIGHTEEN")]
        Eighteen,
    }

    impl ::std::fmt::Display for AppInfoAttributesAustraliaAgeRating {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Fifteen => f.write_str("FIFTEEN"),
                Self::Eighteen => f.write_str("EIGHTEEN"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfoAttributesAustraliaAgeRating {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "FIFTEEN" => Ok(Self::Fifteen),
                "EIGHTEEN" => Ok(Self::Eighteen),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppInfoAttributesAustraliaAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppInfoAttributesAustraliaAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppInfoAttributesAustraliaAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfoAttributesBrazilAgeRatingV2`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "deprecated": true,
    ///  "type": "string",
    ///  "enum": [
    ///    "SELF_RATED_L",
    ///    "SELF_RATED_TEN",
    ///    "SELF_RATED_TWELVE",
    ///    "SELF_RATED_FOURTEEN",
    ///    "SELF_RATED_SIXTEEN",
    ///    "SELF_RATED_EIGHTEEN",
    ///    "OFFICIAL_L",
    ///    "OFFICIAL_TEN",
    ///    "OFFICIAL_TWELVE",
    ///    "OFFICIAL_FOURTEEN",
    ///    "OFFICIAL_SIXTEEN",
    ///    "OFFICIAL_EIGHTEEN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfoAttributesBrazilAgeRatingV2 {
        #[serde(rename = "SELF_RATED_L")]
        SelfRatedL,
        #[serde(rename = "SELF_RATED_TEN")]
        SelfRatedTen,
        #[serde(rename = "SELF_RATED_TWELVE")]
        SelfRatedTwelve,
        #[serde(rename = "SELF_RATED_FOURTEEN")]
        SelfRatedFourteen,
        #[serde(rename = "SELF_RATED_SIXTEEN")]
        SelfRatedSixteen,
        #[serde(rename = "SELF_RATED_EIGHTEEN")]
        SelfRatedEighteen,
        #[serde(rename = "OFFICIAL_L")]
        OfficialL,
        #[serde(rename = "OFFICIAL_TEN")]
        OfficialTen,
        #[serde(rename = "OFFICIAL_TWELVE")]
        OfficialTwelve,
        #[serde(rename = "OFFICIAL_FOURTEEN")]
        OfficialFourteen,
        #[serde(rename = "OFFICIAL_SIXTEEN")]
        OfficialSixteen,
        #[serde(rename = "OFFICIAL_EIGHTEEN")]
        OfficialEighteen,
    }

    impl ::std::fmt::Display for AppInfoAttributesBrazilAgeRatingV2 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SelfRatedL => f.write_str("SELF_RATED_L"),
                Self::SelfRatedTen => f.write_str("SELF_RATED_TEN"),
                Self::SelfRatedTwelve => f.write_str("SELF_RATED_TWELVE"),
                Self::SelfRatedFourteen => f.write_str("SELF_RATED_FOURTEEN"),
                Self::SelfRatedSixteen => f.write_str("SELF_RATED_SIXTEEN"),
                Self::SelfRatedEighteen => f.write_str("SELF_RATED_EIGHTEEN"),
                Self::OfficialL => f.write_str("OFFICIAL_L"),
                Self::OfficialTen => f.write_str("OFFICIAL_TEN"),
                Self::OfficialTwelve => f.write_str("OFFICIAL_TWELVE"),
                Self::OfficialFourteen => f.write_str("OFFICIAL_FOURTEEN"),
                Self::OfficialSixteen => f.write_str("OFFICIAL_SIXTEEN"),
                Self::OfficialEighteen => f.write_str("OFFICIAL_EIGHTEEN"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfoAttributesBrazilAgeRatingV2 {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "SELF_RATED_L" => Ok(Self::SelfRatedL),
                "SELF_RATED_TEN" => Ok(Self::SelfRatedTen),
                "SELF_RATED_TWELVE" => Ok(Self::SelfRatedTwelve),
                "SELF_RATED_FOURTEEN" => Ok(Self::SelfRatedFourteen),
                "SELF_RATED_SIXTEEN" => Ok(Self::SelfRatedSixteen),
                "SELF_RATED_EIGHTEEN" => Ok(Self::SelfRatedEighteen),
                "OFFICIAL_L" => Ok(Self::OfficialL),
                "OFFICIAL_TEN" => Ok(Self::OfficialTen),
                "OFFICIAL_TWELVE" => Ok(Self::OfficialTwelve),
                "OFFICIAL_FOURTEEN" => Ok(Self::OfficialFourteen),
                "OFFICIAL_SIXTEEN" => Ok(Self::OfficialSixteen),
                "OFFICIAL_EIGHTEEN" => Ok(Self::OfficialEighteen),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppInfoAttributesBrazilAgeRatingV2 {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppInfoAttributesBrazilAgeRatingV2 {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppInfoAttributesBrazilAgeRatingV2 {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfoAttributesFranceAgeRating`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "deprecated": true,
    ///  "type": "string",
    ///  "enum": [
    ///    "EIGHTEEN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfoAttributesFranceAgeRating {
        #[serde(rename = "EIGHTEEN")]
        Eighteen,
    }

    impl ::std::fmt::Display for AppInfoAttributesFranceAgeRating {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Eighteen => f.write_str("EIGHTEEN"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfoAttributesFranceAgeRating {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EIGHTEEN" => Ok(Self::Eighteen),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppInfoAttributesFranceAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppInfoAttributesFranceAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppInfoAttributesFranceAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfoAttributesKoreaAgeRating`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "deprecated": true,
    ///  "type": "string",
    ///  "enum": [
    ///    "ALL",
    ///    "TWELVE",
    ///    "FIFTEEN",
    ///    "NINETEEN",
    ///    "NOT_APPLICABLE"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfoAttributesKoreaAgeRating {
        #[serde(rename = "ALL")]
        All,
        #[serde(rename = "TWELVE")]
        Twelve,
        #[serde(rename = "FIFTEEN")]
        Fifteen,
        #[serde(rename = "NINETEEN")]
        Nineteen,
        #[serde(rename = "NOT_APPLICABLE")]
        NotApplicable,
    }

    impl ::std::fmt::Display for AppInfoAttributesKoreaAgeRating {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::All => f.write_str("ALL"),
                Self::Twelve => f.write_str("TWELVE"),
                Self::Fifteen => f.write_str("FIFTEEN"),
                Self::Nineteen => f.write_str("NINETEEN"),
                Self::NotApplicable => f.write_str("NOT_APPLICABLE"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfoAttributesKoreaAgeRating {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ALL" => Ok(Self::All),
                "TWELVE" => Ok(Self::Twelve),
                "FIFTEEN" => Ok(Self::Fifteen),
                "NINETEEN" => Ok(Self::Nineteen),
                "NOT_APPLICABLE" => Ok(Self::NotApplicable),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppInfoAttributesKoreaAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppInfoAttributesKoreaAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppInfoAttributesKoreaAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfoAttributesState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACCEPTED",
    ///    "DEVELOPER_REJECTED",
    ///    "IN_REVIEW",
    ///    "PENDING_RELEASE",
    ///    "PREPARE_FOR_SUBMISSION",
    ///    "READY_FOR_DISTRIBUTION",
    ///    "READY_FOR_REVIEW",
    ///    "REJECTED",
    ///    "REPLACED_WITH_NEW_INFO",
    ///    "WAITING_FOR_REVIEW"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfoAttributesState {
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "DEVELOPER_REJECTED")]
        DeveloperRejected,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "PENDING_RELEASE")]
        PendingRelease,
        #[serde(rename = "PREPARE_FOR_SUBMISSION")]
        PrepareForSubmission,
        #[serde(rename = "READY_FOR_DISTRIBUTION")]
        ReadyForDistribution,
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "REPLACED_WITH_NEW_INFO")]
        ReplacedWithNewInfo,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
    }

    impl ::std::fmt::Display for AppInfoAttributesState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::DeveloperRejected => f.write_str("DEVELOPER_REJECTED"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::PendingRelease => f.write_str("PENDING_RELEASE"),
                Self::PrepareForSubmission => f.write_str("PREPARE_FOR_SUBMISSION"),
                Self::ReadyForDistribution => f.write_str("READY_FOR_DISTRIBUTION"),
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::ReplacedWithNewInfo => f.write_str("REPLACED_WITH_NEW_INFO"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfoAttributesState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACCEPTED" => Ok(Self::Accepted),
                "DEVELOPER_REJECTED" => Ok(Self::DeveloperRejected),
                "IN_REVIEW" => Ok(Self::InReview),
                "PENDING_RELEASE" => Ok(Self::PendingRelease),
                "PREPARE_FOR_SUBMISSION" => Ok(Self::PrepareForSubmission),
                "READY_FOR_DISTRIBUTION" => Ok(Self::ReadyForDistribution),
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "REJECTED" => Ok(Self::Rejected),
                "REPLACED_WITH_NEW_INFO" => Ok(Self::ReplacedWithNewInfo),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppInfoAttributesState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppInfoAttributesState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppInfoAttributesState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfoLocalization`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppInfoLocalization",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "locale": {
    ///          "type": "string"
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "privacyChoicesUrl": {
    ///          "type": "string"
    ///        },
    ///        "privacyPolicyText": {
    ///          "type": "string"
    ///        },
    ///        "privacyPolicyUrl": {
    ///          "type": "string"
    ///        },
    ///        "subtitle": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "relationships": {
    ///      "type": "object",
    ///      "properties": {
    ///        "appInfo": {
    ///          "type": "object",
    ///          "properties": {
    ///            "data": {
    ///              "type": "object",
    ///              "required": [
    ///                "id",
    ///                "type"
    ///              ],
    ///              "properties": {
    ///                "id": {
    ///                  "type": "string"
    ///                },
    ///                "type": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "appInfos"
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appInfoLocalizations"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfoLocalization {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppInfoLocalizationAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub relationships: ::std::option::Option<AppInfoLocalizationRelationships>,
        #[serde(rename = "type")]
        pub type_: AppInfoLocalizationType,
    }

    ///`AppInfoLocalizationAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "locale": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "privacyChoicesUrl": {
    ///      "type": "string"
    ///    },
    ///    "privacyPolicyText": {
    ///      "type": "string"
    ///    },
    ///    "privacyPolicyUrl": {
    ///      "type": "string"
    ///    },
    ///    "subtitle": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfoLocalizationAttributes {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub locale: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "privacyChoicesUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub privacy_choices_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "privacyPolicyText",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub privacy_policy_text: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "privacyPolicyUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub privacy_policy_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub subtitle: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for AppInfoLocalizationAttributes {
        fn default() -> Self {
            Self {
                locale: Default::default(),
                name: Default::default(),
                privacy_choices_url: Default::default(),
                privacy_policy_text: Default::default(),
                privacy_policy_url: Default::default(),
                subtitle: Default::default(),
            }
        }
    }

    ///`AppInfoLocalizationRelationships`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "appInfo": {
    ///      "type": "object",
    ///      "properties": {
    ///        "data": {
    ///          "type": "object",
    ///          "required": [
    ///            "id",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "id": {
    ///              "type": "string"
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "appInfos"
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfoLocalizationRelationships {
        #[serde(
            rename = "appInfo",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_info: ::std::option::Option<AppInfoLocalizationRelationshipsAppInfo>,
    }

    impl ::std::default::Default for AppInfoLocalizationRelationships {
        fn default() -> Self {
            Self {
                app_info: Default::default(),
            }
        }
    }

    ///`AppInfoLocalizationRelationshipsAppInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "appInfos"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfoLocalizationRelationshipsAppInfo {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<AppInfoLocalizationRelationshipsAppInfoData>,
    }

    impl ::std::default::Default for AppInfoLocalizationRelationshipsAppInfo {
        fn default() -> Self {
            Self {
                data: Default::default(),
            }
        }
    }

    ///`AppInfoLocalizationRelationshipsAppInfoData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appInfos"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfoLocalizationRelationshipsAppInfoData {
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppInfoLocalizationRelationshipsAppInfoDataType,
    }

    ///`AppInfoLocalizationRelationshipsAppInfoDataType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appInfos"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfoLocalizationRelationshipsAppInfoDataType {
        #[serde(rename = "appInfos")]
        AppInfos,
    }

    impl ::std::fmt::Display for AppInfoLocalizationRelationshipsAppInfoDataType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppInfos => f.write_str("appInfos"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfoLocalizationRelationshipsAppInfoDataType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appInfos" => Ok(Self::AppInfos),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppInfoLocalizationRelationshipsAppInfoDataType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppInfoLocalizationRelationshipsAppInfoDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppInfoLocalizationRelationshipsAppInfoDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfoLocalizationResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppInfoLocalizationResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "links"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "$ref": "#/components/schemas/AppInfoLocalization"
    ///    },
    ///    "included": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AppInfo"
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/DocumentLinks"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfoLocalizationResponse {
        pub data: AppInfoLocalization,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub included: ::std::vec::Vec<AppInfo>,
        pub links: DocumentLinks,
    }

    ///`AppInfoLocalizationType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appInfoLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfoLocalizationType {
        #[serde(rename = "appInfoLocalizations")]
        AppInfoLocalizations,
    }

    impl ::std::fmt::Display for AppInfoLocalizationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppInfoLocalizations => f.write_str("appInfoLocalizations"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfoLocalizationType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appInfoLocalizations" => Ok(Self::AppInfoLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppInfoLocalizationType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppInfoLocalizationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppInfoLocalizationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfoLocalizationUpdateRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppInfoLocalizationUpdateRequest",
    ///  "type": "object",
    ///  "required": [
    ///    "data"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "attributes": {
    ///          "type": "object",
    ///          "properties": {
    ///            "name": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "privacyChoicesUrl": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "privacyPolicyText": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "privacyPolicyUrl": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "subtitle": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "appInfoLocalizations"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfoLocalizationUpdateRequest {
        pub data: AppInfoLocalizationUpdateRequestData,
    }

    ///`AppInfoLocalizationUpdateRequestData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "name": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "privacyChoicesUrl": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "privacyPolicyText": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "privacyPolicyUrl": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "subtitle": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appInfoLocalizations"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfoLocalizationUpdateRequestData {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppInfoLocalizationUpdateRequestDataAttributes>,
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppInfoLocalizationUpdateRequestDataType,
    }

    ///`AppInfoLocalizationUpdateRequestDataAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "privacyChoicesUrl": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "privacyPolicyText": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "privacyPolicyUrl": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "subtitle": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfoLocalizationUpdateRequestDataAttributes {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "privacyChoicesUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub privacy_choices_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "privacyPolicyText",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub privacy_policy_text: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "privacyPolicyUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub privacy_policy_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub subtitle: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for AppInfoLocalizationUpdateRequestDataAttributes {
        fn default() -> Self {
            Self {
                name: Default::default(),
                privacy_choices_url: Default::default(),
                privacy_policy_text: Default::default(),
                privacy_policy_url: Default::default(),
                subtitle: Default::default(),
            }
        }
    }

    ///`AppInfoLocalizationUpdateRequestDataType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appInfoLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfoLocalizationUpdateRequestDataType {
        #[serde(rename = "appInfoLocalizations")]
        AppInfoLocalizations,
    }

    impl ::std::fmt::Display for AppInfoLocalizationUpdateRequestDataType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppInfoLocalizations => f.write_str("appInfoLocalizations"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfoLocalizationUpdateRequestDataType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appInfoLocalizations" => Ok(Self::AppInfoLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppInfoLocalizationUpdateRequestDataType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppInfoLocalizationUpdateRequestDataType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppInfoLocalizationUpdateRequestDataType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfoLocalizationsGetInstanceFieldsAppInfoLocalizationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "locale",
    ///    "name",
    ///    "subtitle",
    ///    "privacyPolicyUrl",
    ///    "privacyChoicesUrl",
    ///    "privacyPolicyText",
    ///    "appInfo"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfoLocalizationsGetInstanceFieldsAppInfoLocalizationsItem {
        #[serde(rename = "locale")]
        Locale,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "subtitle")]
        Subtitle,
        #[serde(rename = "privacyPolicyUrl")]
        PrivacyPolicyUrl,
        #[serde(rename = "privacyChoicesUrl")]
        PrivacyChoicesUrl,
        #[serde(rename = "privacyPolicyText")]
        PrivacyPolicyText,
        #[serde(rename = "appInfo")]
        AppInfo,
    }

    impl ::std::fmt::Display for AppInfoLocalizationsGetInstanceFieldsAppInfoLocalizationsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Locale => f.write_str("locale"),
                Self::Name => f.write_str("name"),
                Self::Subtitle => f.write_str("subtitle"),
                Self::PrivacyPolicyUrl => f.write_str("privacyPolicyUrl"),
                Self::PrivacyChoicesUrl => f.write_str("privacyChoicesUrl"),
                Self::PrivacyPolicyText => f.write_str("privacyPolicyText"),
                Self::AppInfo => f.write_str("appInfo"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfoLocalizationsGetInstanceFieldsAppInfoLocalizationsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "locale" => Ok(Self::Locale),
                "name" => Ok(Self::Name),
                "subtitle" => Ok(Self::Subtitle),
                "privacyPolicyUrl" => Ok(Self::PrivacyPolicyUrl),
                "privacyChoicesUrl" => Ok(Self::PrivacyChoicesUrl),
                "privacyPolicyText" => Ok(Self::PrivacyPolicyText),
                "appInfo" => Ok(Self::AppInfo),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppInfoLocalizationsGetInstanceFieldsAppInfoLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppInfoLocalizationsGetInstanceFieldsAppInfoLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppInfoLocalizationsGetInstanceFieldsAppInfoLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfoLocalizationsGetInstanceFieldsAppInfosItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreState",
    ///    "state",
    ///    "appStoreAgeRating",
    ///    "australiaAgeRating",
    ///    "brazilAgeRating",
    ///    "brazilAgeRatingV2",
    ///    "franceAgeRating",
    ///    "koreaAgeRating",
    ///    "app",
    ///    "ageRatingDeclaration",
    ///    "appInfoLocalizations",
    ///    "primaryCategory",
    ///    "primarySubcategoryOne",
    ///    "primarySubcategoryTwo",
    ///    "secondaryCategory",
    ///    "secondarySubcategoryOne",
    ///    "secondarySubcategoryTwo",
    ///    "territoryAgeRatings"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfoLocalizationsGetInstanceFieldsAppInfosItem {
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "appStoreAgeRating")]
        AppStoreAgeRating,
        #[serde(rename = "australiaAgeRating")]
        AustraliaAgeRating,
        #[serde(rename = "brazilAgeRating")]
        BrazilAgeRating,
        #[serde(rename = "brazilAgeRatingV2")]
        BrazilAgeRatingV2,
        #[serde(rename = "franceAgeRating")]
        FranceAgeRating,
        #[serde(rename = "koreaAgeRating")]
        KoreaAgeRating,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "ageRatingDeclaration")]
        AgeRatingDeclaration,
        #[serde(rename = "appInfoLocalizations")]
        AppInfoLocalizations,
        #[serde(rename = "primaryCategory")]
        PrimaryCategory,
        #[serde(rename = "primarySubcategoryOne")]
        PrimarySubcategoryOne,
        #[serde(rename = "primarySubcategoryTwo")]
        PrimarySubcategoryTwo,
        #[serde(rename = "secondaryCategory")]
        SecondaryCategory,
        #[serde(rename = "secondarySubcategoryOne")]
        SecondarySubcategoryOne,
        #[serde(rename = "secondarySubcategoryTwo")]
        SecondarySubcategoryTwo,
        #[serde(rename = "territoryAgeRatings")]
        TerritoryAgeRatings,
    }

    impl ::std::fmt::Display for AppInfoLocalizationsGetInstanceFieldsAppInfosItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::State => f.write_str("state"),
                Self::AppStoreAgeRating => f.write_str("appStoreAgeRating"),
                Self::AustraliaAgeRating => f.write_str("australiaAgeRating"),
                Self::BrazilAgeRating => f.write_str("brazilAgeRating"),
                Self::BrazilAgeRatingV2 => f.write_str("brazilAgeRatingV2"),
                Self::FranceAgeRating => f.write_str("franceAgeRating"),
                Self::KoreaAgeRating => f.write_str("koreaAgeRating"),
                Self::App => f.write_str("app"),
                Self::AgeRatingDeclaration => f.write_str("ageRatingDeclaration"),
                Self::AppInfoLocalizations => f.write_str("appInfoLocalizations"),
                Self::PrimaryCategory => f.write_str("primaryCategory"),
                Self::PrimarySubcategoryOne => f.write_str("primarySubcategoryOne"),
                Self::PrimarySubcategoryTwo => f.write_str("primarySubcategoryTwo"),
                Self::SecondaryCategory => f.write_str("secondaryCategory"),
                Self::SecondarySubcategoryOne => f.write_str("secondarySubcategoryOne"),
                Self::SecondarySubcategoryTwo => f.write_str("secondarySubcategoryTwo"),
                Self::TerritoryAgeRatings => f.write_str("territoryAgeRatings"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfoLocalizationsGetInstanceFieldsAppInfosItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreState" => Ok(Self::AppStoreState),
                "state" => Ok(Self::State),
                "appStoreAgeRating" => Ok(Self::AppStoreAgeRating),
                "australiaAgeRating" => Ok(Self::AustraliaAgeRating),
                "brazilAgeRating" => Ok(Self::BrazilAgeRating),
                "brazilAgeRatingV2" => Ok(Self::BrazilAgeRatingV2),
                "franceAgeRating" => Ok(Self::FranceAgeRating),
                "koreaAgeRating" => Ok(Self::KoreaAgeRating),
                "app" => Ok(Self::App),
                "ageRatingDeclaration" => Ok(Self::AgeRatingDeclaration),
                "appInfoLocalizations" => Ok(Self::AppInfoLocalizations),
                "primaryCategory" => Ok(Self::PrimaryCategory),
                "primarySubcategoryOne" => Ok(Self::PrimarySubcategoryOne),
                "primarySubcategoryTwo" => Ok(Self::PrimarySubcategoryTwo),
                "secondaryCategory" => Ok(Self::SecondaryCategory),
                "secondarySubcategoryOne" => Ok(Self::SecondarySubcategoryOne),
                "secondarySubcategoryTwo" => Ok(Self::SecondarySubcategoryTwo),
                "territoryAgeRatings" => Ok(Self::TerritoryAgeRatings),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppInfoLocalizationsGetInstanceFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppInfoLocalizationsGetInstanceFieldsAppInfosItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppInfoLocalizationsGetInstanceFieldsAppInfosItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfoLocalizationsGetInstanceIncludeItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appInfo"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfoLocalizationsGetInstanceIncludeItem {
        #[serde(rename = "appInfo")]
        AppInfo,
    }

    impl ::std::fmt::Display for AppInfoLocalizationsGetInstanceIncludeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppInfo => f.write_str("appInfo"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfoLocalizationsGetInstanceIncludeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appInfo" => Ok(Self::AppInfo),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppInfoLocalizationsGetInstanceIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppInfoLocalizationsGetInstanceIncludeItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppInfoLocalizationsGetInstanceIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfoLocalizationsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppInfoLocalizationsResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "links"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AppInfoLocalization"
    ///      }
    ///    },
    ///    "included": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AppInfo"
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/PagedDocumentLinks"
    ///    },
    ///    "meta": {
    ///      "$ref": "#/components/schemas/PagingInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfoLocalizationsResponse {
        pub data: ::std::vec::Vec<AppInfoLocalization>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub included: ::std::vec::Vec<AppInfo>,
        pub links: PagedDocumentLinks,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub meta: ::std::option::Option<PagingInformation>,
    }

    ///`AppInfoType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appInfos"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfoType {
        #[serde(rename = "appInfos")]
        AppInfos,
    }

    impl ::std::fmt::Display for AppInfoType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppInfos => f.write_str("appInfos"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfoType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appInfos" => Ok(Self::AppInfos),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppInfoType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppInfoType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppInfoType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfoLocalizationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "locale",
    ///    "name",
    ///    "subtitle",
    ///    "privacyPolicyUrl",
    ///    "privacyChoicesUrl",
    ///    "privacyPolicyText",
    ///    "appInfo"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfoLocalizationsItem {
        #[serde(rename = "locale")]
        Locale,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "subtitle")]
        Subtitle,
        #[serde(rename = "privacyPolicyUrl")]
        PrivacyPolicyUrl,
        #[serde(rename = "privacyChoicesUrl")]
        PrivacyChoicesUrl,
        #[serde(rename = "privacyPolicyText")]
        PrivacyPolicyText,
        #[serde(rename = "appInfo")]
        AppInfo,
    }

    impl ::std::fmt::Display
        for AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfoLocalizationsItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Locale => f.write_str("locale"),
                Self::Name => f.write_str("name"),
                Self::Subtitle => f.write_str("subtitle"),
                Self::PrivacyPolicyUrl => f.write_str("privacyPolicyUrl"),
                Self::PrivacyChoicesUrl => f.write_str("privacyChoicesUrl"),
                Self::PrivacyPolicyText => f.write_str("privacyPolicyText"),
                Self::AppInfo => f.write_str("appInfo"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfoLocalizationsItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "locale" => Ok(Self::Locale),
                "name" => Ok(Self::Name),
                "subtitle" => Ok(Self::Subtitle),
                "privacyPolicyUrl" => Ok(Self::PrivacyPolicyUrl),
                "privacyChoicesUrl" => Ok(Self::PrivacyChoicesUrl),
                "privacyPolicyText" => Ok(Self::PrivacyPolicyText),
                "appInfo" => Ok(Self::AppInfo),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfoLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfoLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfoLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfosItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreState",
    ///    "state",
    ///    "appStoreAgeRating",
    ///    "australiaAgeRating",
    ///    "brazilAgeRating",
    ///    "brazilAgeRatingV2",
    ///    "franceAgeRating",
    ///    "koreaAgeRating",
    ///    "app",
    ///    "ageRatingDeclaration",
    ///    "appInfoLocalizations",
    ///    "primaryCategory",
    ///    "primarySubcategoryOne",
    ///    "primarySubcategoryTwo",
    ///    "secondaryCategory",
    ///    "secondarySubcategoryOne",
    ///    "secondarySubcategoryTwo",
    ///    "territoryAgeRatings"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfosItem {
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "appStoreAgeRating")]
        AppStoreAgeRating,
        #[serde(rename = "australiaAgeRating")]
        AustraliaAgeRating,
        #[serde(rename = "brazilAgeRating")]
        BrazilAgeRating,
        #[serde(rename = "brazilAgeRatingV2")]
        BrazilAgeRatingV2,
        #[serde(rename = "franceAgeRating")]
        FranceAgeRating,
        #[serde(rename = "koreaAgeRating")]
        KoreaAgeRating,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "ageRatingDeclaration")]
        AgeRatingDeclaration,
        #[serde(rename = "appInfoLocalizations")]
        AppInfoLocalizations,
        #[serde(rename = "primaryCategory")]
        PrimaryCategory,
        #[serde(rename = "primarySubcategoryOne")]
        PrimarySubcategoryOne,
        #[serde(rename = "primarySubcategoryTwo")]
        PrimarySubcategoryTwo,
        #[serde(rename = "secondaryCategory")]
        SecondaryCategory,
        #[serde(rename = "secondarySubcategoryOne")]
        SecondarySubcategoryOne,
        #[serde(rename = "secondarySubcategoryTwo")]
        SecondarySubcategoryTwo,
        #[serde(rename = "territoryAgeRatings")]
        TerritoryAgeRatings,
    }

    impl ::std::fmt::Display for AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfosItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::State => f.write_str("state"),
                Self::AppStoreAgeRating => f.write_str("appStoreAgeRating"),
                Self::AustraliaAgeRating => f.write_str("australiaAgeRating"),
                Self::BrazilAgeRating => f.write_str("brazilAgeRating"),
                Self::BrazilAgeRatingV2 => f.write_str("brazilAgeRatingV2"),
                Self::FranceAgeRating => f.write_str("franceAgeRating"),
                Self::KoreaAgeRating => f.write_str("koreaAgeRating"),
                Self::App => f.write_str("app"),
                Self::AgeRatingDeclaration => f.write_str("ageRatingDeclaration"),
                Self::AppInfoLocalizations => f.write_str("appInfoLocalizations"),
                Self::PrimaryCategory => f.write_str("primaryCategory"),
                Self::PrimarySubcategoryOne => f.write_str("primarySubcategoryOne"),
                Self::PrimarySubcategoryTwo => f.write_str("primarySubcategoryTwo"),
                Self::SecondaryCategory => f.write_str("secondaryCategory"),
                Self::SecondarySubcategoryOne => f.write_str("secondarySubcategoryOne"),
                Self::SecondarySubcategoryTwo => f.write_str("secondarySubcategoryTwo"),
                Self::TerritoryAgeRatings => f.write_str("territoryAgeRatings"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfosItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreState" => Ok(Self::AppStoreState),
                "state" => Ok(Self::State),
                "appStoreAgeRating" => Ok(Self::AppStoreAgeRating),
                "australiaAgeRating" => Ok(Self::AustraliaAgeRating),
                "brazilAgeRating" => Ok(Self::BrazilAgeRating),
                "brazilAgeRatingV2" => Ok(Self::BrazilAgeRatingV2),
                "franceAgeRating" => Ok(Self::FranceAgeRating),
                "koreaAgeRating" => Ok(Self::KoreaAgeRating),
                "app" => Ok(Self::App),
                "ageRatingDeclaration" => Ok(Self::AgeRatingDeclaration),
                "appInfoLocalizations" => Ok(Self::AppInfoLocalizations),
                "primaryCategory" => Ok(Self::PrimaryCategory),
                "primarySubcategoryOne" => Ok(Self::PrimarySubcategoryOne),
                "primarySubcategoryTwo" => Ok(Self::PrimarySubcategoryTwo),
                "secondaryCategory" => Ok(Self::SecondaryCategory),
                "secondarySubcategoryOne" => Ok(Self::SecondarySubcategoryOne),
                "secondarySubcategoryTwo" => Ok(Self::SecondarySubcategoryTwo),
                "territoryAgeRatings" => Ok(Self::TerritoryAgeRatings),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfosItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfosItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfosItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfosAppInfoLocalizationsGetToManyRelatedIncludeItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appInfo"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppInfosAppInfoLocalizationsGetToManyRelatedIncludeItem {
        #[serde(rename = "appInfo")]
        AppInfo,
    }

    impl ::std::fmt::Display for AppInfosAppInfoLocalizationsGetToManyRelatedIncludeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppInfo => f.write_str("appInfo"),
            }
        }
    }

    impl ::std::str::FromStr for AppInfosAppInfoLocalizationsGetToManyRelatedIncludeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appInfo" => Ok(Self::AppInfo),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppInfosAppInfoLocalizationsGetToManyRelatedIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppInfosAppInfoLocalizationsGetToManyRelatedIncludeItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppInfosAppInfoLocalizationsGetToManyRelatedIncludeItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppInfosResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppInfosResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "links"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AppInfo"
    ///      }
    ///    },
    ///    "included": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "$ref": "#/components/schemas/AgeRatingDeclaration"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/AppCategory"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/AppInfoLocalization"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/App"
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/PagedDocumentLinks"
    ///    },
    ///    "meta": {
    ///      "$ref": "#/components/schemas/PagingInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppInfosResponse {
        pub data: ::std::vec::Vec<AppInfo>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub included: ::std::vec::Vec<AppInfosResponseIncludedItem>,
        pub links: PagedDocumentLinks,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub meta: ::std::option::Option<PagingInformation>,
    }

    ///`AppInfosResponseIncludedItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AgeRatingDeclaration"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppCategory"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppInfoLocalization"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/App"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum AppInfosResponseIncludedItem {
        AgeRatingDeclaration(AgeRatingDeclaration),
        AppCategory(AppCategory),
        AppInfoLocalization(AppInfoLocalization),
        App(App),
    }

    impl ::std::convert::From<AgeRatingDeclaration> for AppInfosResponseIncludedItem {
        fn from(value: AgeRatingDeclaration) -> Self {
            Self::AgeRatingDeclaration(value)
        }
    }

    impl ::std::convert::From<AppCategory> for AppInfosResponseIncludedItem {
        fn from(value: AppCategory) -> Self {
            Self::AppCategory(value)
        }
    }

    impl ::std::convert::From<AppInfoLocalization> for AppInfosResponseIncludedItem {
        fn from(value: AppInfoLocalization) -> Self {
            Self::AppInfoLocalization(value)
        }
    }

    impl ::std::convert::From<App> for AppInfosResponseIncludedItem {
        fn from(value: App) -> Self {
            Self::App(value)
        }
    }

    ///`AppKeyword`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppKeyword",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appKeywords"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppKeyword {
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(rename = "type")]
        pub type_: AppKeywordType,
    }

    ///`AppKeywordType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appKeywords"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppKeywordType {
        #[serde(rename = "appKeywords")]
        AppKeywords,
    }

    impl ::std::fmt::Display for AppKeywordType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppKeywords => f.write_str("appKeywords"),
            }
        }
    }

    impl ::std::str::FromStr for AppKeywordType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appKeywords" => Ok(Self::AppKeywords),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppKeywordType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppKeywordType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppKeywordType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppPreviewSet`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppPreviewSet",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "previewType": {
    ///          "$ref": "#/components/schemas/PreviewType"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "relationships": {
    ///      "type": "object",
    ///      "properties": {
    ///        "appCustomProductPageLocalization": {
    ///          "type": "object",
    ///          "properties": {
    ///            "data": {
    ///              "type": "object",
    ///              "required": [
    ///                "id",
    ///                "type"
    ///              ],
    ///              "properties": {
    ///                "id": {
    ///                  "type": "string"
    ///                },
    ///                "type": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "appCustomProductPageLocalizations"
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "appPreviews": {
    ///          "type": "object",
    ///          "properties": {
    ///            "data": {
    ///              "type": "array",
    ///              "items": {
    ///                "type": "object",
    ///                "required": [
    ///                  "id",
    ///                  "type"
    ///                ],
    ///                "properties": {
    ///                  "id": {
    ///                    "type": "string"
    ///                  },
    ///                  "type": {
    ///                    "type": "string",
    ///                    "enum": [
    ///                      "appPreviews"
    ///                    ]
    ///                  }
    ///                }
    ///              }
    ///            },
    ///            "links": {
    ///              "$ref": "#/components/schemas/RelationshipLinks"
    ///            },
    ///            "meta": {
    ///              "$ref": "#/components/schemas/PagingInformation"
    ///            }
    ///          }
    ///        },
    ///        "appStoreVersionExperimentTreatmentLocalization": {
    ///          "type": "object",
    ///          "properties": {
    ///            "data": {
    ///              "type": "object",
    ///              "required": [
    ///                "id",
    ///                "type"
    ///              ],
    ///              "properties": {
    ///                "id": {
    ///                  "type": "string"
    ///                },
    ///                "type": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "appStoreVersionExperimentTreatmentLocalizations"
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "appStoreVersionLocalization": {
    ///          "type": "object",
    ///          "properties": {
    ///            "data": {
    ///              "type": "object",
    ///              "required": [
    ///                "id",
    ///                "type"
    ///              ],
    ///              "properties": {
    ///                "id": {
    ///                  "type": "string"
    ///                },
    ///                "type": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "appStoreVersionLocalizations"
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appPreviewSets"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppPreviewSet {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppPreviewSetAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub relationships: ::std::option::Option<AppPreviewSetRelationships>,
        #[serde(rename = "type")]
        pub type_: AppPreviewSetType,
    }

    ///`AppPreviewSetAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "previewType": {
    ///      "$ref": "#/components/schemas/PreviewType"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppPreviewSetAttributes {
        #[serde(
            rename = "previewType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub preview_type: ::std::option::Option<PreviewType>,
    }

    impl ::std::default::Default for AppPreviewSetAttributes {
        fn default() -> Self {
            Self {
                preview_type: Default::default(),
            }
        }
    }

    ///`AppPreviewSetRelationships`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "appCustomProductPageLocalization": {
    ///      "type": "object",
    ///      "properties": {
    ///        "data": {
    ///          "type": "object",
    ///          "required": [
    ///            "id",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "id": {
    ///              "type": "string"
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "appCustomProductPageLocalizations"
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "appPreviews": {
    ///      "type": "object",
    ///      "properties": {
    ///        "data": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "object",
    ///            "required": [
    ///              "id",
    ///              "type"
    ///            ],
    ///            "properties": {
    ///              "id": {
    ///                "type": "string"
    ///              },
    ///              "type": {
    ///                "type": "string",
    ///                "enum": [
    ///                  "appPreviews"
    ///                ]
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "links": {
    ///          "$ref": "#/components/schemas/RelationshipLinks"
    ///        },
    ///        "meta": {
    ///          "$ref": "#/components/schemas/PagingInformation"
    ///        }
    ///      }
    ///    },
    ///    "appStoreVersionExperimentTreatmentLocalization": {
    ///      "type": "object",
    ///      "properties": {
    ///        "data": {
    ///          "type": "object",
    ///          "required": [
    ///            "id",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "id": {
    ///              "type": "string"
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "appStoreVersionExperimentTreatmentLocalizations"
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "appStoreVersionLocalization": {
    ///      "type": "object",
    ///      "properties": {
    ///        "data": {
    ///          "type": "object",
    ///          "required": [
    ///            "id",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "id": {
    ///              "type": "string"
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "appStoreVersionLocalizations"
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppPreviewSetRelationships {
        #[serde(
            rename = "appCustomProductPageLocalization",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_custom_product_page_localization:
            ::std::option::Option<AppPreviewSetRelationshipsAppCustomProductPageLocalization>,
        #[serde(
            rename = "appPreviews",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_previews: ::std::option::Option<AppPreviewSetRelationshipsAppPreviews>,
        #[serde(
            rename = "appStoreVersionExperimentTreatmentLocalization",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_store_version_experiment_treatment_localization: ::std::option::Option<
            AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalization,
        >,
        #[serde(
            rename = "appStoreVersionLocalization",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_store_version_localization:
            ::std::option::Option<AppPreviewSetRelationshipsAppStoreVersionLocalization>,
    }

    impl ::std::default::Default for AppPreviewSetRelationships {
        fn default() -> Self {
            Self {
                app_custom_product_page_localization: Default::default(),
                app_previews: Default::default(),
                app_store_version_experiment_treatment_localization: Default::default(),
                app_store_version_localization: Default::default(),
            }
        }
    }

    ///`AppPreviewSetRelationshipsAppCustomProductPageLocalization`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "appCustomProductPageLocalizations"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppPreviewSetRelationshipsAppCustomProductPageLocalization {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data:
            ::std::option::Option<AppPreviewSetRelationshipsAppCustomProductPageLocalizationData>,
    }

    impl ::std::default::Default for AppPreviewSetRelationshipsAppCustomProductPageLocalization {
        fn default() -> Self {
            Self {
                data: Default::default(),
            }
        }
    }

    ///`AppPreviewSetRelationshipsAppCustomProductPageLocalizationData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appCustomProductPageLocalizations"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppPreviewSetRelationshipsAppCustomProductPageLocalizationData {
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppPreviewSetRelationshipsAppCustomProductPageLocalizationDataType,
    }

    ///`AppPreviewSetRelationshipsAppCustomProductPageLocalizationDataType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appCustomProductPageLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppPreviewSetRelationshipsAppCustomProductPageLocalizationDataType {
        #[serde(rename = "appCustomProductPageLocalizations")]
        AppCustomProductPageLocalizations,
    }

    impl ::std::fmt::Display for AppPreviewSetRelationshipsAppCustomProductPageLocalizationDataType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppCustomProductPageLocalizations => {
                    f.write_str("appCustomProductPageLocalizations")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppPreviewSetRelationshipsAppCustomProductPageLocalizationDataType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appCustomProductPageLocalizations" => Ok(Self::AppCustomProductPageLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppPreviewSetRelationshipsAppCustomProductPageLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppPreviewSetRelationshipsAppCustomProductPageLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppPreviewSetRelationshipsAppCustomProductPageLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppPreviewSetRelationshipsAppPreviews`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "id",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "id": {
    ///            "type": "string"
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "appPreviews"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/RelationshipLinks"
    ///    },
    ///    "meta": {
    ///      "$ref": "#/components/schemas/PagingInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppPreviewSetRelationshipsAppPreviews {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub data: ::std::vec::Vec<AppPreviewSetRelationshipsAppPreviewsDataItem>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<RelationshipLinks>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub meta: ::std::option::Option<PagingInformation>,
    }

    impl ::std::default::Default for AppPreviewSetRelationshipsAppPreviews {
        fn default() -> Self {
            Self {
                data: Default::default(),
                links: Default::default(),
                meta: Default::default(),
            }
        }
    }

    ///`AppPreviewSetRelationshipsAppPreviewsDataItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appPreviews"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppPreviewSetRelationshipsAppPreviewsDataItem {
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppPreviewSetRelationshipsAppPreviewsDataItemType,
    }

    ///`AppPreviewSetRelationshipsAppPreviewsDataItemType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appPreviews"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppPreviewSetRelationshipsAppPreviewsDataItemType {
        #[serde(rename = "appPreviews")]
        AppPreviews,
    }

    impl ::std::fmt::Display for AppPreviewSetRelationshipsAppPreviewsDataItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppPreviews => f.write_str("appPreviews"),
            }
        }
    }

    impl ::std::str::FromStr for AppPreviewSetRelationshipsAppPreviewsDataItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appPreviews" => Ok(Self::AppPreviews),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppPreviewSetRelationshipsAppPreviewsDataItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppPreviewSetRelationshipsAppPreviewsDataItemType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppPreviewSetRelationshipsAppPreviewsDataItemType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalization`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "appStoreVersionExperimentTreatmentLocalizations"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalization {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<
            AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationData,
        >,
    }

    impl ::std::default::Default
        for AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalization
    {
        fn default() -> Self {
            Self {
                data: Default::default(),
            }
        }
    }

    ///`AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appStoreVersionExperimentTreatmentLocalizations"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationData {
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType,
    }

    ///`AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersionExperimentTreatmentLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType {
        #[serde(rename = "appStoreVersionExperimentTreatmentLocalizations")]
        AppStoreVersionExperimentTreatmentLocalizations,
    }

    impl ::std::fmt::Display
        for AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersionExperimentTreatmentLocalizations => {
                    f.write_str("appStoreVersionExperimentTreatmentLocalizations")
                }
            }
        }
    }

    impl ::std::str::FromStr
        for AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersionExperimentTreatmentLocalizations" => {
                    Ok(Self::AppStoreVersionExperimentTreatmentLocalizations)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppPreviewSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppPreviewSetRelationshipsAppStoreVersionLocalization`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "appStoreVersionLocalizations"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppPreviewSetRelationshipsAppStoreVersionLocalization {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<AppPreviewSetRelationshipsAppStoreVersionLocalizationData>,
    }

    impl ::std::default::Default for AppPreviewSetRelationshipsAppStoreVersionLocalization {
        fn default() -> Self {
            Self {
                data: Default::default(),
            }
        }
    }

    ///`AppPreviewSetRelationshipsAppStoreVersionLocalizationData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appStoreVersionLocalizations"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppPreviewSetRelationshipsAppStoreVersionLocalizationData {
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppPreviewSetRelationshipsAppStoreVersionLocalizationDataType,
    }

    ///`AppPreviewSetRelationshipsAppStoreVersionLocalizationDataType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersionLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppPreviewSetRelationshipsAppStoreVersionLocalizationDataType {
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
    }

    impl ::std::fmt::Display for AppPreviewSetRelationshipsAppStoreVersionLocalizationDataType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
            }
        }
    }

    impl ::std::str::FromStr for AppPreviewSetRelationshipsAppStoreVersionLocalizationDataType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppPreviewSetRelationshipsAppStoreVersionLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppPreviewSetRelationshipsAppStoreVersionLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppPreviewSetRelationshipsAppStoreVersionLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppPreviewSetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appPreviewSets"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppPreviewSetType {
        #[serde(rename = "appPreviewSets")]
        AppPreviewSets,
    }

    impl ::std::fmt::Display for AppPreviewSetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppPreviewSets => f.write_str("appPreviewSets"),
            }
        }
    }

    impl ::std::str::FromStr for AppPreviewSetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appPreviewSets" => Ok(Self::AppPreviewSets),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppPreviewSetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppPreviewSetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppPreviewSetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "links"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "$ref": "#/components/schemas/App"
    ///    },
    ///    "included": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "$ref": "#/components/schemas/Build"
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/DocumentLinks"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppResponse {
        pub data: App,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub included: ::std::vec::Vec<Build>,
        pub links: DocumentLinks,
    }

    ///`AppScreenshotSet`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppScreenshotSet",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "screenshotDisplayType": {
    ///          "$ref": "#/components/schemas/ScreenshotDisplayType"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "relationships": {
    ///      "type": "object",
    ///      "properties": {
    ///        "appCustomProductPageLocalization": {
    ///          "type": "object",
    ///          "properties": {
    ///            "data": {
    ///              "type": "object",
    ///              "required": [
    ///                "id",
    ///                "type"
    ///              ],
    ///              "properties": {
    ///                "id": {
    ///                  "type": "string"
    ///                },
    ///                "type": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "appCustomProductPageLocalizations"
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "appScreenshots": {
    ///          "type": "object",
    ///          "properties": {
    ///            "data": {
    ///              "type": "array",
    ///              "items": {
    ///                "type": "object",
    ///                "required": [
    ///                  "id",
    ///                  "type"
    ///                ],
    ///                "properties": {
    ///                  "id": {
    ///                    "type": "string"
    ///                  },
    ///                  "type": {
    ///                    "type": "string",
    ///                    "enum": [
    ///                      "appScreenshots"
    ///                    ]
    ///                  }
    ///                }
    ///              }
    ///            },
    ///            "links": {
    ///              "$ref": "#/components/schemas/RelationshipLinks"
    ///            },
    ///            "meta": {
    ///              "$ref": "#/components/schemas/PagingInformation"
    ///            }
    ///          }
    ///        },
    ///        "appStoreVersionExperimentTreatmentLocalization": {
    ///          "type": "object",
    ///          "properties": {
    ///            "data": {
    ///              "type": "object",
    ///              "required": [
    ///                "id",
    ///                "type"
    ///              ],
    ///              "properties": {
    ///                "id": {
    ///                  "type": "string"
    ///                },
    ///                "type": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "appStoreVersionExperimentTreatmentLocalizations"
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "appStoreVersionLocalization": {
    ///          "type": "object",
    ///          "properties": {
    ///            "data": {
    ///              "type": "object",
    ///              "required": [
    ///                "id",
    ///                "type"
    ///              ],
    ///              "properties": {
    ///                "id": {
    ///                  "type": "string"
    ///                },
    ///                "type": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "appStoreVersionLocalizations"
    ///                  ]
    ///                }
    ///              }
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appScreenshotSets"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppScreenshotSet {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppScreenshotSetAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub relationships: ::std::option::Option<AppScreenshotSetRelationships>,
        #[serde(rename = "type")]
        pub type_: AppScreenshotSetType,
    }

    ///`AppScreenshotSetAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "screenshotDisplayType": {
    ///      "$ref": "#/components/schemas/ScreenshotDisplayType"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppScreenshotSetAttributes {
        #[serde(
            rename = "screenshotDisplayType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub screenshot_display_type: ::std::option::Option<ScreenshotDisplayType>,
    }

    impl ::std::default::Default for AppScreenshotSetAttributes {
        fn default() -> Self {
            Self {
                screenshot_display_type: Default::default(),
            }
        }
    }

    ///`AppScreenshotSetRelationships`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "appCustomProductPageLocalization": {
    ///      "type": "object",
    ///      "properties": {
    ///        "data": {
    ///          "type": "object",
    ///          "required": [
    ///            "id",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "id": {
    ///              "type": "string"
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "appCustomProductPageLocalizations"
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "appScreenshots": {
    ///      "type": "object",
    ///      "properties": {
    ///        "data": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "object",
    ///            "required": [
    ///              "id",
    ///              "type"
    ///            ],
    ///            "properties": {
    ///              "id": {
    ///                "type": "string"
    ///              },
    ///              "type": {
    ///                "type": "string",
    ///                "enum": [
    ///                  "appScreenshots"
    ///                ]
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "links": {
    ///          "$ref": "#/components/schemas/RelationshipLinks"
    ///        },
    ///        "meta": {
    ///          "$ref": "#/components/schemas/PagingInformation"
    ///        }
    ///      }
    ///    },
    ///    "appStoreVersionExperimentTreatmentLocalization": {
    ///      "type": "object",
    ///      "properties": {
    ///        "data": {
    ///          "type": "object",
    ///          "required": [
    ///            "id",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "id": {
    ///              "type": "string"
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "appStoreVersionExperimentTreatmentLocalizations"
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "appStoreVersionLocalization": {
    ///      "type": "object",
    ///      "properties": {
    ///        "data": {
    ///          "type": "object",
    ///          "required": [
    ///            "id",
    ///            "type"
    ///          ],
    ///          "properties": {
    ///            "id": {
    ///              "type": "string"
    ///            },
    ///            "type": {
    ///              "type": "string",
    ///              "enum": [
    ///                "appStoreVersionLocalizations"
    ///              ]
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppScreenshotSetRelationships {
        #[serde(
            rename = "appCustomProductPageLocalization",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_custom_product_page_localization:
            ::std::option::Option<AppScreenshotSetRelationshipsAppCustomProductPageLocalization>,
        #[serde(
            rename = "appScreenshots",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_screenshots: ::std::option::Option<AppScreenshotSetRelationshipsAppScreenshots>,
        #[serde(
            rename = "appStoreVersionExperimentTreatmentLocalization",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_store_version_experiment_treatment_localization: ::std::option::Option<
            AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalization,
        >,
        #[serde(
            rename = "appStoreVersionLocalization",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_store_version_localization:
            ::std::option::Option<AppScreenshotSetRelationshipsAppStoreVersionLocalization>,
    }

    impl ::std::default::Default for AppScreenshotSetRelationships {
        fn default() -> Self {
            Self {
                app_custom_product_page_localization: Default::default(),
                app_screenshots: Default::default(),
                app_store_version_experiment_treatment_localization: Default::default(),
                app_store_version_localization: Default::default(),
            }
        }
    }

    ///`AppScreenshotSetRelationshipsAppCustomProductPageLocalization`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "appCustomProductPageLocalizations"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppScreenshotSetRelationshipsAppCustomProductPageLocalization {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<
            AppScreenshotSetRelationshipsAppCustomProductPageLocalizationData,
        >,
    }

    impl ::std::default::Default for AppScreenshotSetRelationshipsAppCustomProductPageLocalization {
        fn default() -> Self {
            Self {
                data: Default::default(),
            }
        }
    }

    ///`AppScreenshotSetRelationshipsAppCustomProductPageLocalizationData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appCustomProductPageLocalizations"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppScreenshotSetRelationshipsAppCustomProductPageLocalizationData {
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppScreenshotSetRelationshipsAppCustomProductPageLocalizationDataType,
    }

    ///`AppScreenshotSetRelationshipsAppCustomProductPageLocalizationDataType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appCustomProductPageLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppScreenshotSetRelationshipsAppCustomProductPageLocalizationDataType {
        #[serde(rename = "appCustomProductPageLocalizations")]
        AppCustomProductPageLocalizations,
    }

    impl ::std::fmt::Display for AppScreenshotSetRelationshipsAppCustomProductPageLocalizationDataType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppCustomProductPageLocalizations => {
                    f.write_str("appCustomProductPageLocalizations")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppScreenshotSetRelationshipsAppCustomProductPageLocalizationDataType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appCustomProductPageLocalizations" => Ok(Self::AppCustomProductPageLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppScreenshotSetRelationshipsAppCustomProductPageLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppScreenshotSetRelationshipsAppCustomProductPageLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppScreenshotSetRelationshipsAppCustomProductPageLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppScreenshotSetRelationshipsAppScreenshots`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "id",
    ///          "type"
    ///        ],
    ///        "properties": {
    ///          "id": {
    ///            "type": "string"
    ///          },
    ///          "type": {
    ///            "type": "string",
    ///            "enum": [
    ///              "appScreenshots"
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/RelationshipLinks"
    ///    },
    ///    "meta": {
    ///      "$ref": "#/components/schemas/PagingInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppScreenshotSetRelationshipsAppScreenshots {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub data: ::std::vec::Vec<AppScreenshotSetRelationshipsAppScreenshotsDataItem>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<RelationshipLinks>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub meta: ::std::option::Option<PagingInformation>,
    }

    impl ::std::default::Default for AppScreenshotSetRelationshipsAppScreenshots {
        fn default() -> Self {
            Self {
                data: Default::default(),
                links: Default::default(),
                meta: Default::default(),
            }
        }
    }

    ///`AppScreenshotSetRelationshipsAppScreenshotsDataItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appScreenshots"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppScreenshotSetRelationshipsAppScreenshotsDataItem {
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppScreenshotSetRelationshipsAppScreenshotsDataItemType,
    }

    ///`AppScreenshotSetRelationshipsAppScreenshotsDataItemType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appScreenshots"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppScreenshotSetRelationshipsAppScreenshotsDataItemType {
        #[serde(rename = "appScreenshots")]
        AppScreenshots,
    }

    impl ::std::fmt::Display for AppScreenshotSetRelationshipsAppScreenshotsDataItemType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppScreenshots => f.write_str("appScreenshots"),
            }
        }
    }

    impl ::std::str::FromStr for AppScreenshotSetRelationshipsAppScreenshotsDataItemType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appScreenshots" => Ok(Self::AppScreenshots),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppScreenshotSetRelationshipsAppScreenshotsDataItemType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppScreenshotSetRelationshipsAppScreenshotsDataItemType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppScreenshotSetRelationshipsAppScreenshotsDataItemType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalization`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "appStoreVersionExperimentTreatmentLocalizations"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalization {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<
            AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationData,
        >,
    }

    impl ::std::default::Default
        for AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalization
    {
        fn default() -> Self {
            Self {
                data: Default::default(),
            }
        }
    }

    ///`AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appStoreVersionExperimentTreatmentLocalizations"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationData {
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_:
            AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType,
    }

    ///`AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersionExperimentTreatmentLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType {
        #[serde(rename = "appStoreVersionExperimentTreatmentLocalizations")]
        AppStoreVersionExperimentTreatmentLocalizations,
    }

    impl ::std::fmt::Display
        for AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersionExperimentTreatmentLocalizations => {
                    f.write_str("appStoreVersionExperimentTreatmentLocalizations")
                }
            }
        }
    }

    impl ::std::str::FromStr
        for AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersionExperimentTreatmentLocalizations" => {
                    Ok(Self::AppStoreVersionExperimentTreatmentLocalizations)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppScreenshotSetRelationshipsAppStoreVersionExperimentTreatmentLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppScreenshotSetRelationshipsAppStoreVersionLocalization`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "appStoreVersionLocalizations"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppScreenshotSetRelationshipsAppStoreVersionLocalization {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data:
            ::std::option::Option<AppScreenshotSetRelationshipsAppStoreVersionLocalizationData>,
    }

    impl ::std::default::Default for AppScreenshotSetRelationshipsAppStoreVersionLocalization {
        fn default() -> Self {
            Self {
                data: Default::default(),
            }
        }
    }

    ///`AppScreenshotSetRelationshipsAppStoreVersionLocalizationData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appStoreVersionLocalizations"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppScreenshotSetRelationshipsAppStoreVersionLocalizationData {
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppScreenshotSetRelationshipsAppStoreVersionLocalizationDataType,
    }

    ///`AppScreenshotSetRelationshipsAppStoreVersionLocalizationDataType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersionLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppScreenshotSetRelationshipsAppStoreVersionLocalizationDataType {
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
    }

    impl ::std::fmt::Display for AppScreenshotSetRelationshipsAppStoreVersionLocalizationDataType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
            }
        }
    }

    impl ::std::str::FromStr for AppScreenshotSetRelationshipsAppStoreVersionLocalizationDataType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppScreenshotSetRelationshipsAppStoreVersionLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppScreenshotSetRelationshipsAppStoreVersionLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppScreenshotSetRelationshipsAppStoreVersionLocalizationDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppScreenshotSetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appScreenshotSets"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppScreenshotSetType {
        #[serde(rename = "appScreenshotSets")]
        AppScreenshotSets,
    }

    impl ::std::fmt::Display for AppScreenshotSetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppScreenshotSets => f.write_str("appScreenshotSets"),
            }
        }
    }

    impl ::std::str::FromStr for AppScreenshotSetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appScreenshotSets" => Ok(Self::AppScreenshotSets),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppScreenshotSetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppScreenshotSetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppScreenshotSetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreAgeRating`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "L",
    ///    "ALL",
    ///    "ZERO_ZERO",
    ///    "ONE_PLUS",
    ///    "TWO_PLUS",
    ///    "THREE_PLUS",
    ///    "FOUR_PLUS",
    ///    "FIVE_PLUS",
    ///    "SIX_PLUS",
    ///    "SEVEN_PLUS",
    ///    "EIGHT_PLUS",
    ///    "NINE_PLUS",
    ///    "TEN_PLUS",
    ///    "ELEVEN_PLUS",
    ///    "TWELVE_PLUS",
    ///    "THIRTEEN_PLUS",
    ///    "FOURTEEN_PLUS",
    ///    "FIFTEEN_PLUS",
    ///    "SIXTEEN_PLUS",
    ///    "SEVENTEEN_PLUS",
    ///    "EIGHTEEN_PLUS",
    ///    "NINETEEN_PLUS",
    ///    "TWENTY_PLUS",
    ///    "TWENTY_ONE_PLUS",
    ///    "UNRATED"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreAgeRating {
        L,
        #[serde(rename = "ALL")]
        All,
        #[serde(rename = "ZERO_ZERO")]
        ZeroZero,
        #[serde(rename = "ONE_PLUS")]
        OnePlus,
        #[serde(rename = "TWO_PLUS")]
        TwoPlus,
        #[serde(rename = "THREE_PLUS")]
        ThreePlus,
        #[serde(rename = "FOUR_PLUS")]
        FourPlus,
        #[serde(rename = "FIVE_PLUS")]
        FivePlus,
        #[serde(rename = "SIX_PLUS")]
        SixPlus,
        #[serde(rename = "SEVEN_PLUS")]
        SevenPlus,
        #[serde(rename = "EIGHT_PLUS")]
        EightPlus,
        #[serde(rename = "NINE_PLUS")]
        NinePlus,
        #[serde(rename = "TEN_PLUS")]
        TenPlus,
        #[serde(rename = "ELEVEN_PLUS")]
        ElevenPlus,
        #[serde(rename = "TWELVE_PLUS")]
        TwelvePlus,
        #[serde(rename = "THIRTEEN_PLUS")]
        ThirteenPlus,
        #[serde(rename = "FOURTEEN_PLUS")]
        FourteenPlus,
        #[serde(rename = "FIFTEEN_PLUS")]
        FifteenPlus,
        #[serde(rename = "SIXTEEN_PLUS")]
        SixteenPlus,
        #[serde(rename = "SEVENTEEN_PLUS")]
        SeventeenPlus,
        #[serde(rename = "EIGHTEEN_PLUS")]
        EighteenPlus,
        #[serde(rename = "NINETEEN_PLUS")]
        NineteenPlus,
        #[serde(rename = "TWENTY_PLUS")]
        TwentyPlus,
        #[serde(rename = "TWENTY_ONE_PLUS")]
        TwentyOnePlus,
        #[serde(rename = "UNRATED")]
        Unrated,
    }

    impl ::std::fmt::Display for AppStoreAgeRating {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::L => f.write_str("L"),
                Self::All => f.write_str("ALL"),
                Self::ZeroZero => f.write_str("ZERO_ZERO"),
                Self::OnePlus => f.write_str("ONE_PLUS"),
                Self::TwoPlus => f.write_str("TWO_PLUS"),
                Self::ThreePlus => f.write_str("THREE_PLUS"),
                Self::FourPlus => f.write_str("FOUR_PLUS"),
                Self::FivePlus => f.write_str("FIVE_PLUS"),
                Self::SixPlus => f.write_str("SIX_PLUS"),
                Self::SevenPlus => f.write_str("SEVEN_PLUS"),
                Self::EightPlus => f.write_str("EIGHT_PLUS"),
                Self::NinePlus => f.write_str("NINE_PLUS"),
                Self::TenPlus => f.write_str("TEN_PLUS"),
                Self::ElevenPlus => f.write_str("ELEVEN_PLUS"),
                Self::TwelvePlus => f.write_str("TWELVE_PLUS"),
                Self::ThirteenPlus => f.write_str("THIRTEEN_PLUS"),
                Self::FourteenPlus => f.write_str("FOURTEEN_PLUS"),
                Self::FifteenPlus => f.write_str("FIFTEEN_PLUS"),
                Self::SixteenPlus => f.write_str("SIXTEEN_PLUS"),
                Self::SeventeenPlus => f.write_str("SEVENTEEN_PLUS"),
                Self::EighteenPlus => f.write_str("EIGHTEEN_PLUS"),
                Self::NineteenPlus => f.write_str("NINETEEN_PLUS"),
                Self::TwentyPlus => f.write_str("TWENTY_PLUS"),
                Self::TwentyOnePlus => f.write_str("TWENTY_ONE_PLUS"),
                Self::Unrated => f.write_str("UNRATED"),
            }
        }
    }

    impl ::std::str::FromStr for AppStoreAgeRating {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "L" => Ok(Self::L),
                "ALL" => Ok(Self::All),
                "ZERO_ZERO" => Ok(Self::ZeroZero),
                "ONE_PLUS" => Ok(Self::OnePlus),
                "TWO_PLUS" => Ok(Self::TwoPlus),
                "THREE_PLUS" => Ok(Self::ThreePlus),
                "FOUR_PLUS" => Ok(Self::FourPlus),
                "FIVE_PLUS" => Ok(Self::FivePlus),
                "SIX_PLUS" => Ok(Self::SixPlus),
                "SEVEN_PLUS" => Ok(Self::SevenPlus),
                "EIGHT_PLUS" => Ok(Self::EightPlus),
                "NINE_PLUS" => Ok(Self::NinePlus),
                "TEN_PLUS" => Ok(Self::TenPlus),
                "ELEVEN_PLUS" => Ok(Self::ElevenPlus),
                "TWELVE_PLUS" => Ok(Self::TwelvePlus),
                "THIRTEEN_PLUS" => Ok(Self::ThirteenPlus),
                "FOURTEEN_PLUS" => Ok(Self::FourteenPlus),
                "FIFTEEN_PLUS" => Ok(Self::FifteenPlus),
                "SIXTEEN_PLUS" => Ok(Self::SixteenPlus),
                "SEVENTEEN_PLUS" => Ok(Self::SeventeenPlus),
                "EIGHTEEN_PLUS" => Ok(Self::EighteenPlus),
                "NINETEEN_PLUS" => Ok(Self::NineteenPlus),
                "TWENTY_PLUS" => Ok(Self::TwentyPlus),
                "TWENTY_ONE_PLUS" => Ok(Self::TwentyOnePlus),
                "UNRATED" => Ok(Self::Unrated),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppStoreAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppStoreAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppStoreAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersion`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppStoreVersion",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "appStoreState": {
    ///          "$ref": "#/components/schemas/AppStoreVersionState"
    ///        },
    ///        "createdDate": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "earliestReleaseDate": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "versionString": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appStoreVersions"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersion {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppStoreVersionAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(rename = "type")]
        pub type_: AppStoreVersionType,
    }

    ///`AppStoreVersionAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "appStoreState": {
    ///      "$ref": "#/components/schemas/AppStoreVersionState"
    ///    },
    ///    "createdDate": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "earliestReleaseDate": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "versionString": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionAttributes {
        #[serde(
            rename = "appStoreState",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_store_state: ::std::option::Option<AppStoreVersionState>,
        #[serde(
            rename = "createdDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "earliestReleaseDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub earliest_release_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "versionString",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub version_string: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for AppStoreVersionAttributes {
        fn default() -> Self {
            Self {
                app_store_state: Default::default(),
                created_date: Default::default(),
                earliest_release_date: Default::default(),
                version_string: Default::default(),
            }
        }
    }

    ///`AppStoreVersionLocalization`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppStoreVersionLocalization",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "description": {
    ///          "type": "string"
    ///        },
    ///        "keywords": {
    ///          "type": "string"
    ///        },
    ///        "locale": {
    ///          "type": "string"
    ///        },
    ///        "marketingUrl": {
    ///          "type": "string",
    ///          "format": "uri"
    ///        },
    ///        "promotionalText": {
    ///          "type": "string"
    ///        },
    ///        "supportUrl": {
    ///          "type": "string",
    ///          "format": "uri"
    ///        },
    ///        "whatsNew": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appStoreVersionLocalizations"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionLocalization {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppStoreVersionLocalizationAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(rename = "type")]
        pub type_: AppStoreVersionLocalizationType,
    }

    ///`AppStoreVersionLocalizationAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "keywords": {
    ///      "type": "string"
    ///    },
    ///    "locale": {
    ///      "type": "string"
    ///    },
    ///    "marketingUrl": {
    ///      "type": "string",
    ///      "format": "uri"
    ///    },
    ///    "promotionalText": {
    ///      "type": "string"
    ///    },
    ///    "supportUrl": {
    ///      "type": "string",
    ///      "format": "uri"
    ///    },
    ///    "whatsNew": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionLocalizationAttributes {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub keywords: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub locale: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "marketingUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub marketing_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "promotionalText",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub promotional_text: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "supportUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub support_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "whatsNew",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub whats_new: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for AppStoreVersionLocalizationAttributes {
        fn default() -> Self {
            Self {
                description: Default::default(),
                keywords: Default::default(),
                locale: Default::default(),
                marketing_url: Default::default(),
                promotional_text: Default::default(),
                support_url: Default::default(),
                whats_new: Default::default(),
            }
        }
    }

    ///`AppStoreVersionLocalizationResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppStoreVersionLocalizationResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "links"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "$ref": "#/components/schemas/AppStoreVersionLocalization"
    ///    },
    ///    "included": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "$ref": "#/components/schemas/AppKeyword"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/AppPreviewSet"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/AppScreenshotSet"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/AppStoreVersion"
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/DocumentLinks"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionLocalizationResponse {
        pub data: AppStoreVersionLocalization,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub included: ::std::vec::Vec<AppStoreVersionLocalizationResponseIncludedItem>,
        pub links: DocumentLinks,
    }

    ///`AppStoreVersionLocalizationResponseIncludedItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AppKeyword"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppPreviewSet"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppScreenshotSet"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppStoreVersion"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum AppStoreVersionLocalizationResponseIncludedItem {
        Keyword(AppKeyword),
        PreviewSet(AppPreviewSet),
        ScreenshotSet(AppScreenshotSet),
        StoreVersion(AppStoreVersion),
    }

    impl ::std::convert::From<AppKeyword> for AppStoreVersionLocalizationResponseIncludedItem {
        fn from(value: AppKeyword) -> Self {
            Self::Keyword(value)
        }
    }

    impl ::std::convert::From<AppPreviewSet> for AppStoreVersionLocalizationResponseIncludedItem {
        fn from(value: AppPreviewSet) -> Self {
            Self::PreviewSet(value)
        }
    }

    impl ::std::convert::From<AppScreenshotSet> for AppStoreVersionLocalizationResponseIncludedItem {
        fn from(value: AppScreenshotSet) -> Self {
            Self::ScreenshotSet(value)
        }
    }

    impl ::std::convert::From<AppStoreVersion> for AppStoreVersionLocalizationResponseIncludedItem {
        fn from(value: AppStoreVersion) -> Self {
            Self::StoreVersion(value)
        }
    }

    ///`AppStoreVersionLocalizationType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersionLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionLocalizationType {
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
    }

    impl ::std::fmt::Display for AppStoreVersionLocalizationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
            }
        }
    }

    impl ::std::str::FromStr for AppStoreVersionLocalizationType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppStoreVersionLocalizationType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppStoreVersionLocalizationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppStoreVersionLocalizationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionLocalizationUpdateRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppStoreVersionLocalizationUpdateRequest",
    ///  "type": "object",
    ///  "required": [
    ///    "data"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "attributes": {
    ///          "type": "object",
    ///          "properties": {
    ///            "description": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "keywords": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "marketingUrl": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ],
    ///              "format": "uri"
    ///            },
    ///            "promotionalText": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "supportUrl": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ],
    ///              "format": "uri"
    ///            },
    ///            "whatsNew": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "appStoreVersionLocalizations"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionLocalizationUpdateRequest {
        pub data: AppStoreVersionLocalizationUpdateRequestData,
    }

    ///`AppStoreVersionLocalizationUpdateRequestData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "description": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "keywords": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "marketingUrl": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "uri"
    ///        },
    ///        "promotionalText": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "supportUrl": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "uri"
    ///        },
    ///        "whatsNew": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appStoreVersionLocalizations"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionLocalizationUpdateRequestData {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes:
            ::std::option::Option<AppStoreVersionLocalizationUpdateRequestDataAttributes>,
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppStoreVersionLocalizationUpdateRequestDataType,
    }

    ///`AppStoreVersionLocalizationUpdateRequestDataAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "keywords": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "marketingUrl": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uri"
    ///    },
    ///    "promotionalText": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "supportUrl": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uri"
    ///    },
    ///    "whatsNew": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionLocalizationUpdateRequestDataAttributes {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub keywords: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "marketingUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub marketing_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "promotionalText",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub promotional_text: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "supportUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub support_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "whatsNew",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub whats_new: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for AppStoreVersionLocalizationUpdateRequestDataAttributes {
        fn default() -> Self {
            Self {
                description: Default::default(),
                keywords: Default::default(),
                marketing_url: Default::default(),
                promotional_text: Default::default(),
                support_url: Default::default(),
                whats_new: Default::default(),
            }
        }
    }

    ///`AppStoreVersionLocalizationUpdateRequestDataType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersionLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionLocalizationUpdateRequestDataType {
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
    }

    impl ::std::fmt::Display for AppStoreVersionLocalizationUpdateRequestDataType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
            }
        }
    }

    impl ::std::str::FromStr for AppStoreVersionLocalizationUpdateRequestDataType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppStoreVersionLocalizationUpdateRequestDataType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppStoreVersionLocalizationUpdateRequestDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppStoreVersionLocalizationUpdateRequestDataType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionLocalizationsGetInstanceFieldsAppPreviewSetsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "previewType",
    ///    "appStoreVersionLocalization",
    ///    "appCustomProductPageLocalization",
    ///    "appStoreVersionExperimentTreatmentLocalization",
    ///    "appPreviews"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionLocalizationsGetInstanceFieldsAppPreviewSetsItem {
        #[serde(rename = "previewType")]
        PreviewType,
        #[serde(rename = "appStoreVersionLocalization")]
        AppStoreVersionLocalization,
        #[serde(rename = "appCustomProductPageLocalization")]
        AppCustomProductPageLocalization,
        #[serde(rename = "appStoreVersionExperimentTreatmentLocalization")]
        AppStoreVersionExperimentTreatmentLocalization,
        #[serde(rename = "appPreviews")]
        AppPreviews,
    }

    impl ::std::fmt::Display for AppStoreVersionLocalizationsGetInstanceFieldsAppPreviewSetsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::PreviewType => f.write_str("previewType"),
                Self::AppStoreVersionLocalization => f.write_str("appStoreVersionLocalization"),
                Self::AppCustomProductPageLocalization => {
                    f.write_str("appCustomProductPageLocalization")
                }
                Self::AppStoreVersionExperimentTreatmentLocalization => {
                    f.write_str("appStoreVersionExperimentTreatmentLocalization")
                }
                Self::AppPreviews => f.write_str("appPreviews"),
            }
        }
    }

    impl ::std::str::FromStr for AppStoreVersionLocalizationsGetInstanceFieldsAppPreviewSetsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "previewType" => Ok(Self::PreviewType),
                "appStoreVersionLocalization" => Ok(Self::AppStoreVersionLocalization),
                "appCustomProductPageLocalization" => Ok(Self::AppCustomProductPageLocalization),
                "appStoreVersionExperimentTreatmentLocalization" => {
                    Ok(Self::AppStoreVersionExperimentTreatmentLocalization)
                }
                "appPreviews" => Ok(Self::AppPreviews),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppStoreVersionLocalizationsGetInstanceFieldsAppPreviewSetsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppStoreVersionLocalizationsGetInstanceFieldsAppPreviewSetsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppStoreVersionLocalizationsGetInstanceFieldsAppPreviewSetsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionLocalizationsGetInstanceFieldsAppScreenshotSetsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "screenshotDisplayType",
    ///    "appStoreVersionLocalization",
    ///    "appCustomProductPageLocalization",
    ///    "appStoreVersionExperimentTreatmentLocalization",
    ///    "appScreenshots"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionLocalizationsGetInstanceFieldsAppScreenshotSetsItem {
        #[serde(rename = "screenshotDisplayType")]
        ScreenshotDisplayType,
        #[serde(rename = "appStoreVersionLocalization")]
        AppStoreVersionLocalization,
        #[serde(rename = "appCustomProductPageLocalization")]
        AppCustomProductPageLocalization,
        #[serde(rename = "appStoreVersionExperimentTreatmentLocalization")]
        AppStoreVersionExperimentTreatmentLocalization,
        #[serde(rename = "appScreenshots")]
        AppScreenshots,
    }

    impl ::std::fmt::Display for AppStoreVersionLocalizationsGetInstanceFieldsAppScreenshotSetsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ScreenshotDisplayType => f.write_str("screenshotDisplayType"),
                Self::AppStoreVersionLocalization => f.write_str("appStoreVersionLocalization"),
                Self::AppCustomProductPageLocalization => {
                    f.write_str("appCustomProductPageLocalization")
                }
                Self::AppStoreVersionExperimentTreatmentLocalization => {
                    f.write_str("appStoreVersionExperimentTreatmentLocalization")
                }
                Self::AppScreenshots => f.write_str("appScreenshots"),
            }
        }
    }

    impl ::std::str::FromStr for AppStoreVersionLocalizationsGetInstanceFieldsAppScreenshotSetsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "screenshotDisplayType" => Ok(Self::ScreenshotDisplayType),
                "appStoreVersionLocalization" => Ok(Self::AppStoreVersionLocalization),
                "appCustomProductPageLocalization" => Ok(Self::AppCustomProductPageLocalization),
                "appStoreVersionExperimentTreatmentLocalization" => {
                    Ok(Self::AppStoreVersionExperimentTreatmentLocalization)
                }
                "appScreenshots" => Ok(Self::AppScreenshots),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppStoreVersionLocalizationsGetInstanceFieldsAppScreenshotSetsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppStoreVersionLocalizationsGetInstanceFieldsAppScreenshotSetsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppStoreVersionLocalizationsGetInstanceFieldsAppScreenshotSetsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionLocalizationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "description",
    ///    "locale",
    ///    "keywords",
    ///    "marketingUrl",
    ///    "promotionalText",
    ///    "supportUrl",
    ///    "whatsNew",
    ///    "appStoreVersion",
    ///    "appScreenshotSets",
    ///    "appPreviewSets",
    ///    "searchKeywords"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionLocalizationsItem {
        #[serde(rename = "description")]
        Description,
        #[serde(rename = "locale")]
        Locale,
        #[serde(rename = "keywords")]
        Keywords,
        #[serde(rename = "marketingUrl")]
        MarketingUrl,
        #[serde(rename = "promotionalText")]
        PromotionalText,
        #[serde(rename = "supportUrl")]
        SupportUrl,
        #[serde(rename = "whatsNew")]
        WhatsNew,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "appScreenshotSets")]
        AppScreenshotSets,
        #[serde(rename = "appPreviewSets")]
        AppPreviewSets,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
    }

    impl ::std::fmt::Display
        for AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionLocalizationsItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Description => f.write_str("description"),
                Self::Locale => f.write_str("locale"),
                Self::Keywords => f.write_str("keywords"),
                Self::MarketingUrl => f.write_str("marketingUrl"),
                Self::PromotionalText => f.write_str("promotionalText"),
                Self::SupportUrl => f.write_str("supportUrl"),
                Self::WhatsNew => f.write_str("whatsNew"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::AppScreenshotSets => f.write_str("appScreenshotSets"),
                Self::AppPreviewSets => f.write_str("appPreviewSets"),
                Self::SearchKeywords => f.write_str("searchKeywords"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionLocalizationsItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "description" => Ok(Self::Description),
                "locale" => Ok(Self::Locale),
                "keywords" => Ok(Self::Keywords),
                "marketingUrl" => Ok(Self::MarketingUrl),
                "promotionalText" => Ok(Self::PromotionalText),
                "supportUrl" => Ok(Self::SupportUrl),
                "whatsNew" => Ok(Self::WhatsNew),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "appScreenshotSets" => Ok(Self::AppScreenshotSets),
                "appPreviewSets" => Ok(Self::AppPreviewSets),
                "searchKeywords" => Ok(Self::SearchKeywords),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "versionString",
    ///    "appStoreState",
    ///    "appVersionState",
    ///    "copyright",
    ///    "reviewType",
    ///    "releaseType",
    ///    "earliestReleaseDate",
    ///    "usesIdfa",
    ///    "downloadable",
    ///    "createdDate",
    ///    "app",
    ///    "appStoreVersionLocalizations",
    ///    "build",
    ///    "appStoreVersionPhasedRelease",
    ///    "gameCenterAppVersion",
    ///    "routingAppCoverage",
    ///    "appStoreReviewDetail",
    ///    "appStoreVersionSubmission",
    ///    "appClipDefaultExperience",
    ///    "appStoreVersionExperiments",
    ///    "appStoreVersionExperimentsV2",
    ///    "customerReviews",
    ///    "alternativeDistributionPackage"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "versionString")]
        VersionString,
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "appVersionState")]
        AppVersionState,
        #[serde(rename = "copyright")]
        Copyright,
        #[serde(rename = "reviewType")]
        ReviewType,
        #[serde(rename = "releaseType")]
        ReleaseType,
        #[serde(rename = "earliestReleaseDate")]
        EarliestReleaseDate,
        #[serde(rename = "usesIdfa")]
        UsesIdfa,
        #[serde(rename = "downloadable")]
        Downloadable,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "appStoreVersionPhasedRelease")]
        AppStoreVersionPhasedRelease,
        #[serde(rename = "gameCenterAppVersion")]
        GameCenterAppVersion,
        #[serde(rename = "routingAppCoverage")]
        RoutingAppCoverage,
        #[serde(rename = "appStoreReviewDetail")]
        AppStoreReviewDetail,
        #[serde(rename = "appStoreVersionSubmission")]
        AppStoreVersionSubmission,
        #[serde(rename = "appClipDefaultExperience")]
        AppClipDefaultExperience,
        #[serde(rename = "appStoreVersionExperiments")]
        AppStoreVersionExperiments,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "alternativeDistributionPackage")]
        AlternativeDistributionPackage,
    }

    impl ::std::fmt::Display for AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::VersionString => f.write_str("versionString"),
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::AppVersionState => f.write_str("appVersionState"),
                Self::Copyright => f.write_str("copyright"),
                Self::ReviewType => f.write_str("reviewType"),
                Self::ReleaseType => f.write_str("releaseType"),
                Self::EarliestReleaseDate => f.write_str("earliestReleaseDate"),
                Self::UsesIdfa => f.write_str("usesIdfa"),
                Self::Downloadable => f.write_str("downloadable"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::App => f.write_str("app"),
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
                Self::Build => f.write_str("build"),
                Self::AppStoreVersionPhasedRelease => f.write_str("appStoreVersionPhasedRelease"),
                Self::GameCenterAppVersion => f.write_str("gameCenterAppVersion"),
                Self::RoutingAppCoverage => f.write_str("routingAppCoverage"),
                Self::AppStoreReviewDetail => f.write_str("appStoreReviewDetail"),
                Self::AppStoreVersionSubmission => f.write_str("appStoreVersionSubmission"),
                Self::AppClipDefaultExperience => f.write_str("appClipDefaultExperience"),
                Self::AppStoreVersionExperiments => f.write_str("appStoreVersionExperiments"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::AlternativeDistributionPackage => {
                    f.write_str("alternativeDistributionPackage")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "versionString" => Ok(Self::VersionString),
                "appStoreState" => Ok(Self::AppStoreState),
                "appVersionState" => Ok(Self::AppVersionState),
                "copyright" => Ok(Self::Copyright),
                "reviewType" => Ok(Self::ReviewType),
                "releaseType" => Ok(Self::ReleaseType),
                "earliestReleaseDate" => Ok(Self::EarliestReleaseDate),
                "usesIdfa" => Ok(Self::UsesIdfa),
                "downloadable" => Ok(Self::Downloadable),
                "createdDate" => Ok(Self::CreatedDate),
                "app" => Ok(Self::App),
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                "build" => Ok(Self::Build),
                "appStoreVersionPhasedRelease" => Ok(Self::AppStoreVersionPhasedRelease),
                "gameCenterAppVersion" => Ok(Self::GameCenterAppVersion),
                "routingAppCoverage" => Ok(Self::RoutingAppCoverage),
                "appStoreReviewDetail" => Ok(Self::AppStoreReviewDetail),
                "appStoreVersionSubmission" => Ok(Self::AppStoreVersionSubmission),
                "appClipDefaultExperience" => Ok(Self::AppClipDefaultExperience),
                "appStoreVersionExperiments" => Ok(Self::AppStoreVersionExperiments),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "customerReviews" => Ok(Self::CustomerReviews),
                "alternativeDistributionPackage" => Ok(Self::AlternativeDistributionPackage),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionLocalizationsGetInstanceIncludeItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersion",
    ///    "appScreenshotSets",
    ///    "appPreviewSets",
    ///    "searchKeywords"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionLocalizationsGetInstanceIncludeItem {
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "appScreenshotSets")]
        AppScreenshotSets,
        #[serde(rename = "appPreviewSets")]
        AppPreviewSets,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
    }

    impl ::std::fmt::Display for AppStoreVersionLocalizationsGetInstanceIncludeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::AppScreenshotSets => f.write_str("appScreenshotSets"),
                Self::AppPreviewSets => f.write_str("appPreviewSets"),
                Self::SearchKeywords => f.write_str("searchKeywords"),
            }
        }
    }

    impl ::std::str::FromStr for AppStoreVersionLocalizationsGetInstanceIncludeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "appScreenshotSets" => Ok(Self::AppScreenshotSets),
                "appPreviewSets" => Ok(Self::AppPreviewSets),
                "searchKeywords" => Ok(Self::SearchKeywords),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppStoreVersionLocalizationsGetInstanceIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppStoreVersionLocalizationsGetInstanceIncludeItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppStoreVersionLocalizationsGetInstanceIncludeItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionLocalizationsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppStoreVersionLocalizationsResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "links"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AppStoreVersionLocalization"
    ///      }
    ///    },
    ///    "included": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "$ref": "#/components/schemas/AppKeyword"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/AppPreviewSet"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/AppScreenshotSet"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/AppStoreVersion"
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/PagedDocumentLinks"
    ///    },
    ///    "meta": {
    ///      "$ref": "#/components/schemas/PagingInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionLocalizationsResponse {
        pub data: ::std::vec::Vec<AppStoreVersionLocalization>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub included: ::std::vec::Vec<AppStoreVersionLocalizationsResponseIncludedItem>,
        pub links: PagedDocumentLinks,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub meta: ::std::option::Option<PagingInformation>,
    }

    ///`AppStoreVersionLocalizationsResponseIncludedItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AppKeyword"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppPreviewSet"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppScreenshotSet"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AppStoreVersion"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum AppStoreVersionLocalizationsResponseIncludedItem {
        Keyword(AppKeyword),
        PreviewSet(AppPreviewSet),
        ScreenshotSet(AppScreenshotSet),
        StoreVersion(AppStoreVersion),
    }

    impl ::std::convert::From<AppKeyword> for AppStoreVersionLocalizationsResponseIncludedItem {
        fn from(value: AppKeyword) -> Self {
            Self::Keyword(value)
        }
    }

    impl ::std::convert::From<AppPreviewSet> for AppStoreVersionLocalizationsResponseIncludedItem {
        fn from(value: AppPreviewSet) -> Self {
            Self::PreviewSet(value)
        }
    }

    impl ::std::convert::From<AppScreenshotSet> for AppStoreVersionLocalizationsResponseIncludedItem {
        fn from(value: AppScreenshotSet) -> Self {
            Self::ScreenshotSet(value)
        }
    }

    impl ::std::convert::From<AppStoreVersion> for AppStoreVersionLocalizationsResponseIncludedItem {
        fn from(value: AppStoreVersion) -> Self {
            Self::StoreVersion(value)
        }
    }

    ///`AppStoreVersionState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACCEPTED",
    ///    "DEVELOPER_REMOVED_FROM_SALE",
    ///    "DEVELOPER_REJECTED",
    ///    "IN_REVIEW",
    ///    "INVALID_BINARY",
    ///    "METADATA_REJECTED",
    ///    "PENDING_APPLE_RELEASE",
    ///    "PENDING_CONTRACT",
    ///    "PENDING_DEVELOPER_RELEASE",
    ///    "PREPARE_FOR_SUBMISSION",
    ///    "PREORDER_READY_FOR_SALE",
    ///    "PROCESSING_FOR_APP_STORE",
    ///    "READY_FOR_REVIEW",
    ///    "READY_FOR_SALE",
    ///    "REJECTED",
    ///    "REMOVED_FROM_SALE",
    ///    "WAITING_FOR_EXPORT_COMPLIANCE",
    ///    "WAITING_FOR_REVIEW",
    ///    "REPLACED_WITH_NEW_VERSION",
    ///    "NOT_APPLICABLE"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionState {
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "DEVELOPER_REMOVED_FROM_SALE")]
        DeveloperRemovedFromSale,
        #[serde(rename = "DEVELOPER_REJECTED")]
        DeveloperRejected,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "INVALID_BINARY")]
        InvalidBinary,
        #[serde(rename = "METADATA_REJECTED")]
        MetadataRejected,
        #[serde(rename = "PENDING_APPLE_RELEASE")]
        PendingAppleRelease,
        #[serde(rename = "PENDING_CONTRACT")]
        PendingContract,
        #[serde(rename = "PENDING_DEVELOPER_RELEASE")]
        PendingDeveloperRelease,
        #[serde(rename = "PREPARE_FOR_SUBMISSION")]
        PrepareForSubmission,
        #[serde(rename = "PREORDER_READY_FOR_SALE")]
        PreorderReadyForSale,
        #[serde(rename = "PROCESSING_FOR_APP_STORE")]
        ProcessingForAppStore,
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "READY_FOR_SALE")]
        ReadyForSale,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "REMOVED_FROM_SALE")]
        RemovedFromSale,
        #[serde(rename = "WAITING_FOR_EXPORT_COMPLIANCE")]
        WaitingForExportCompliance,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
        #[serde(rename = "REPLACED_WITH_NEW_VERSION")]
        ReplacedWithNewVersion,
        #[serde(rename = "NOT_APPLICABLE")]
        NotApplicable,
    }

    impl ::std::fmt::Display for AppStoreVersionState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::DeveloperRemovedFromSale => f.write_str("DEVELOPER_REMOVED_FROM_SALE"),
                Self::DeveloperRejected => f.write_str("DEVELOPER_REJECTED"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::InvalidBinary => f.write_str("INVALID_BINARY"),
                Self::MetadataRejected => f.write_str("METADATA_REJECTED"),
                Self::PendingAppleRelease => f.write_str("PENDING_APPLE_RELEASE"),
                Self::PendingContract => f.write_str("PENDING_CONTRACT"),
                Self::PendingDeveloperRelease => f.write_str("PENDING_DEVELOPER_RELEASE"),
                Self::PrepareForSubmission => f.write_str("PREPARE_FOR_SUBMISSION"),
                Self::PreorderReadyForSale => f.write_str("PREORDER_READY_FOR_SALE"),
                Self::ProcessingForAppStore => f.write_str("PROCESSING_FOR_APP_STORE"),
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::ReadyForSale => f.write_str("READY_FOR_SALE"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::RemovedFromSale => f.write_str("REMOVED_FROM_SALE"),
                Self::WaitingForExportCompliance => f.write_str("WAITING_FOR_EXPORT_COMPLIANCE"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
                Self::ReplacedWithNewVersion => f.write_str("REPLACED_WITH_NEW_VERSION"),
                Self::NotApplicable => f.write_str("NOT_APPLICABLE"),
            }
        }
    }

    impl ::std::str::FromStr for AppStoreVersionState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACCEPTED" => Ok(Self::Accepted),
                "DEVELOPER_REMOVED_FROM_SALE" => Ok(Self::DeveloperRemovedFromSale),
                "DEVELOPER_REJECTED" => Ok(Self::DeveloperRejected),
                "IN_REVIEW" => Ok(Self::InReview),
                "INVALID_BINARY" => Ok(Self::InvalidBinary),
                "METADATA_REJECTED" => Ok(Self::MetadataRejected),
                "PENDING_APPLE_RELEASE" => Ok(Self::PendingAppleRelease),
                "PENDING_CONTRACT" => Ok(Self::PendingContract),
                "PENDING_DEVELOPER_RELEASE" => Ok(Self::PendingDeveloperRelease),
                "PREPARE_FOR_SUBMISSION" => Ok(Self::PrepareForSubmission),
                "PREORDER_READY_FOR_SALE" => Ok(Self::PreorderReadyForSale),
                "PROCESSING_FOR_APP_STORE" => Ok(Self::ProcessingForAppStore),
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "READY_FOR_SALE" => Ok(Self::ReadyForSale),
                "REJECTED" => Ok(Self::Rejected),
                "REMOVED_FROM_SALE" => Ok(Self::RemovedFromSale),
                "WAITING_FOR_EXPORT_COMPLIANCE" => Ok(Self::WaitingForExportCompliance),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                "REPLACED_WITH_NEW_VERSION" => Ok(Self::ReplacedWithNewVersion),
                "NOT_APPLICABLE" => Ok(Self::NotApplicable),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppStoreVersionState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppStoreVersionState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppStoreVersionState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionType {
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
    }

    impl ::std::fmt::Display for AppStoreVersionType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
            }
        }
    }

    impl ::std::str::FromStr for AppStoreVersionType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppStoreVersionType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppStoreVersionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppStoreVersionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppPreviewSetsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "previewType",
    ///    "appStoreVersionLocalization",
    ///    "appCustomProductPageLocalization",
    ///    "appStoreVersionExperimentTreatmentLocalization",
    ///    "appPreviews"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppPreviewSetsItem {
        #[serde(rename = "previewType")]
        PreviewType,
        #[serde(rename = "appStoreVersionLocalization")]
        AppStoreVersionLocalization,
        #[serde(rename = "appCustomProductPageLocalization")]
        AppCustomProductPageLocalization,
        #[serde(rename = "appStoreVersionExperimentTreatmentLocalization")]
        AppStoreVersionExperimentTreatmentLocalization,
        #[serde(rename = "appPreviews")]
        AppPreviews,
    }

    impl ::std::fmt::Display
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppPreviewSetsItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::PreviewType => f.write_str("previewType"),
                Self::AppStoreVersionLocalization => f.write_str("appStoreVersionLocalization"),
                Self::AppCustomProductPageLocalization => {
                    f.write_str("appCustomProductPageLocalization")
                }
                Self::AppStoreVersionExperimentTreatmentLocalization => {
                    f.write_str("appStoreVersionExperimentTreatmentLocalization")
                }
                Self::AppPreviews => f.write_str("appPreviews"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppPreviewSetsItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "previewType" => Ok(Self::PreviewType),
                "appStoreVersionLocalization" => Ok(Self::AppStoreVersionLocalization),
                "appCustomProductPageLocalization" => Ok(Self::AppCustomProductPageLocalization),
                "appStoreVersionExperimentTreatmentLocalization" => {
                    Ok(Self::AppStoreVersionExperimentTreatmentLocalization)
                }
                "appPreviews" => Ok(Self::AppPreviews),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppPreviewSetsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppPreviewSetsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppPreviewSetsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppScreenshotSetsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "screenshotDisplayType",
    ///    "appStoreVersionLocalization",
    ///    "appCustomProductPageLocalization",
    ///    "appStoreVersionExperimentTreatmentLocalization",
    ///    "appScreenshots"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppScreenshotSetsItem {
        #[serde(rename = "screenshotDisplayType")]
        ScreenshotDisplayType,
        #[serde(rename = "appStoreVersionLocalization")]
        AppStoreVersionLocalization,
        #[serde(rename = "appCustomProductPageLocalization")]
        AppCustomProductPageLocalization,
        #[serde(rename = "appStoreVersionExperimentTreatmentLocalization")]
        AppStoreVersionExperimentTreatmentLocalization,
        #[serde(rename = "appScreenshots")]
        AppScreenshots,
    }

    impl ::std::fmt::Display
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppScreenshotSetsItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ScreenshotDisplayType => f.write_str("screenshotDisplayType"),
                Self::AppStoreVersionLocalization => f.write_str("appStoreVersionLocalization"),
                Self::AppCustomProductPageLocalization => {
                    f.write_str("appCustomProductPageLocalization")
                }
                Self::AppStoreVersionExperimentTreatmentLocalization => {
                    f.write_str("appStoreVersionExperimentTreatmentLocalization")
                }
                Self::AppScreenshots => f.write_str("appScreenshots"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppScreenshotSetsItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "screenshotDisplayType" => Ok(Self::ScreenshotDisplayType),
                "appStoreVersionLocalization" => Ok(Self::AppStoreVersionLocalization),
                "appCustomProductPageLocalization" => Ok(Self::AppCustomProductPageLocalization),
                "appStoreVersionExperimentTreatmentLocalization" => {
                    Ok(Self::AppStoreVersionExperimentTreatmentLocalization)
                }
                "appScreenshots" => Ok(Self::AppScreenshots),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppScreenshotSetsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppScreenshotSetsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppScreenshotSetsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "description",
    ///    "locale",
    ///    "keywords",
    ///    "marketingUrl",
    ///    "promotionalText",
    ///    "supportUrl",
    ///    "whatsNew",
    ///    "appStoreVersion",
    ///    "appScreenshotSets",
    ///    "appPreviewSets",
    ///    "searchKeywords"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem
    {
        #[serde(rename = "description")]
        Description,
        #[serde(rename = "locale")]
        Locale,
        #[serde(rename = "keywords")]
        Keywords,
        #[serde(rename = "marketingUrl")]
        MarketingUrl,
        #[serde(rename = "promotionalText")]
        PromotionalText,
        #[serde(rename = "supportUrl")]
        SupportUrl,
        #[serde(rename = "whatsNew")]
        WhatsNew,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "appScreenshotSets")]
        AppScreenshotSets,
        #[serde(rename = "appPreviewSets")]
        AppPreviewSets,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
    }

    impl :: std :: fmt :: Display for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem { fn fmt (& self , f : & mut :: std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result { match * self { Self :: Description => f . write_str ("description") , Self :: Locale => f . write_str ("locale") , Self :: Keywords => f . write_str ("keywords") , Self :: MarketingUrl => f . write_str ("marketingUrl") , Self :: PromotionalText => f . write_str ("promotionalText") , Self :: SupportUrl => f . write_str ("supportUrl") , Self :: WhatsNew => f . write_str ("whatsNew") , Self :: AppStoreVersion => f . write_str ("appStoreVersion") , Self :: AppScreenshotSets => f . write_str ("appScreenshotSets") , Self :: AppPreviewSets => f . write_str ("appPreviewSets") , Self :: SearchKeywords => f . write_str ("searchKeywords") , } } }
    impl :: std :: str :: FromStr for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem { type Err = self :: error :: ConversionError ; fn from_str (value : & str) -> :: std :: result :: Result < Self , self :: error :: ConversionError > { match value { "description" => Ok (Self :: Description) , "locale" => Ok (Self :: Locale) , "keywords" => Ok (Self :: Keywords) , "marketingUrl" => Ok (Self :: MarketingUrl) , "promotionalText" => Ok (Self :: PromotionalText) , "supportUrl" => Ok (Self :: SupportUrl) , "whatsNew" => Ok (Self :: WhatsNew) , "appStoreVersion" => Ok (Self :: AppStoreVersion) , "appScreenshotSets" => Ok (Self :: AppScreenshotSets) , "appPreviewSets" => Ok (Self :: AppPreviewSets) , "searchKeywords" => Ok (Self :: SearchKeywords) , _ => Err ("invalid value" . into ()) , } } }
    impl :: std :: convert :: TryFrom < & str > for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem { type Error = self :: error :: ConversionError ; fn try_from (value : & str) -> :: std :: result :: Result < Self , self :: error :: ConversionError > { value . parse () } }
    impl :: std :: convert :: TryFrom < & :: std :: string :: String > for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem { type Error = self :: error :: ConversionError ; fn try_from (value : & :: std :: string :: String) -> :: std :: result :: Result < Self , self :: error :: ConversionError > { value . parse () } }
    impl :: std :: convert :: TryFrom < :: std :: string :: String > for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem { type Error = self :: error :: ConversionError ; fn try_from (value : :: std :: string :: String) -> :: std :: result :: Result < Self , self :: error :: ConversionError > { value . parse () } }
    ///`AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "versionString",
    ///    "appStoreState",
    ///    "appVersionState",
    ///    "copyright",
    ///    "reviewType",
    ///    "releaseType",
    ///    "earliestReleaseDate",
    ///    "usesIdfa",
    ///    "downloadable",
    ///    "createdDate",
    ///    "app",
    ///    "appStoreVersionLocalizations",
    ///    "build",
    ///    "appStoreVersionPhasedRelease",
    ///    "gameCenterAppVersion",
    ///    "routingAppCoverage",
    ///    "appStoreReviewDetail",
    ///    "appStoreVersionSubmission",
    ///    "appClipDefaultExperience",
    ///    "appStoreVersionExperiments",
    ///    "appStoreVersionExperimentsV2",
    ///    "customerReviews",
    ///    "alternativeDistributionPackage"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "versionString")]
        VersionString,
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "appVersionState")]
        AppVersionState,
        #[serde(rename = "copyright")]
        Copyright,
        #[serde(rename = "reviewType")]
        ReviewType,
        #[serde(rename = "releaseType")]
        ReleaseType,
        #[serde(rename = "earliestReleaseDate")]
        EarliestReleaseDate,
        #[serde(rename = "usesIdfa")]
        UsesIdfa,
        #[serde(rename = "downloadable")]
        Downloadable,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "appStoreVersionPhasedRelease")]
        AppStoreVersionPhasedRelease,
        #[serde(rename = "gameCenterAppVersion")]
        GameCenterAppVersion,
        #[serde(rename = "routingAppCoverage")]
        RoutingAppCoverage,
        #[serde(rename = "appStoreReviewDetail")]
        AppStoreReviewDetail,
        #[serde(rename = "appStoreVersionSubmission")]
        AppStoreVersionSubmission,
        #[serde(rename = "appClipDefaultExperience")]
        AppClipDefaultExperience,
        #[serde(rename = "appStoreVersionExperiments")]
        AppStoreVersionExperiments,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "alternativeDistributionPackage")]
        AlternativeDistributionPackage,
    }

    impl ::std::fmt::Display
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionsItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::VersionString => f.write_str("versionString"),
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::AppVersionState => f.write_str("appVersionState"),
                Self::Copyright => f.write_str("copyright"),
                Self::ReviewType => f.write_str("reviewType"),
                Self::ReleaseType => f.write_str("releaseType"),
                Self::EarliestReleaseDate => f.write_str("earliestReleaseDate"),
                Self::UsesIdfa => f.write_str("usesIdfa"),
                Self::Downloadable => f.write_str("downloadable"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::App => f.write_str("app"),
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
                Self::Build => f.write_str("build"),
                Self::AppStoreVersionPhasedRelease => f.write_str("appStoreVersionPhasedRelease"),
                Self::GameCenterAppVersion => f.write_str("gameCenterAppVersion"),
                Self::RoutingAppCoverage => f.write_str("routingAppCoverage"),
                Self::AppStoreReviewDetail => f.write_str("appStoreReviewDetail"),
                Self::AppStoreVersionSubmission => f.write_str("appStoreVersionSubmission"),
                Self::AppClipDefaultExperience => f.write_str("appClipDefaultExperience"),
                Self::AppStoreVersionExperiments => f.write_str("appStoreVersionExperiments"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::AlternativeDistributionPackage => {
                    f.write_str("alternativeDistributionPackage")
                }
            }
        }
    }

    impl ::std::str::FromStr
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionsItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "versionString" => Ok(Self::VersionString),
                "appStoreState" => Ok(Self::AppStoreState),
                "appVersionState" => Ok(Self::AppVersionState),
                "copyright" => Ok(Self::Copyright),
                "reviewType" => Ok(Self::ReviewType),
                "releaseType" => Ok(Self::ReleaseType),
                "earliestReleaseDate" => Ok(Self::EarliestReleaseDate),
                "usesIdfa" => Ok(Self::UsesIdfa),
                "downloadable" => Ok(Self::Downloadable),
                "createdDate" => Ok(Self::CreatedDate),
                "app" => Ok(Self::App),
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                "build" => Ok(Self::Build),
                "appStoreVersionPhasedRelease" => Ok(Self::AppStoreVersionPhasedRelease),
                "gameCenterAppVersion" => Ok(Self::GameCenterAppVersion),
                "routingAppCoverage" => Ok(Self::RoutingAppCoverage),
                "appStoreReviewDetail" => Ok(Self::AppStoreReviewDetail),
                "appStoreVersionSubmission" => Ok(Self::AppStoreVersionSubmission),
                "appClipDefaultExperience" => Ok(Self::AppClipDefaultExperience),
                "appStoreVersionExperiments" => Ok(Self::AppStoreVersionExperiments),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "customerReviews" => Ok(Self::CustomerReviews),
                "alternativeDistributionPackage" => Ok(Self::AlternativeDistributionPackage),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedIncludeItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersion",
    ///    "appScreenshotSets",
    ///    "appPreviewSets",
    ///    "searchKeywords"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedIncludeItem {
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "appScreenshotSets")]
        AppScreenshotSets,
        #[serde(rename = "appPreviewSets")]
        AppPreviewSets,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
    }

    impl ::std::fmt::Display
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedIncludeItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::AppScreenshotSets => f.write_str("appScreenshotSets"),
                Self::AppPreviewSets => f.write_str("appPreviewSets"),
                Self::SearchKeywords => f.write_str("searchKeywords"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedIncludeItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "appScreenshotSets" => Ok(Self::AppScreenshotSets),
                "appPreviewSets" => Ok(Self::AppPreviewSets),
                "searchKeywords" => Ok(Self::SearchKeywords),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedIncludeItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedIncludeItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedIncludeItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppStoreVersionsResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "links"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AppStoreVersion"
    ///      }
    ///    },
    ///    "included": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "$ref": "#/components/schemas/AppStoreVersionLocalization"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/Build"
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/PagedDocumentLinks"
    ///    },
    ///    "meta": {
    ///      "$ref": "#/components/schemas/PagingInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionsResponse {
        pub data: ::std::vec::Vec<AppStoreVersion>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub included: ::std::vec::Vec<AppStoreVersionsResponseIncludedItem>,
        pub links: PagedDocumentLinks,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub meta: ::std::option::Option<PagingInformation>,
    }

    ///`AppStoreVersionsResponseIncludedItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AppStoreVersionLocalization"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Build"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum AppStoreVersionsResponseIncludedItem {
        AppStoreVersionLocalization(AppStoreVersionLocalization),
        Build(Build),
    }

    impl ::std::convert::From<AppStoreVersionLocalization> for AppStoreVersionsResponseIncludedItem {
        fn from(value: AppStoreVersionLocalization) -> Self {
            Self::AppStoreVersionLocalization(value)
        }
    }

    impl ::std::convert::From<Build> for AppStoreVersionsResponseIncludedItem {
        fn from(value: Build) -> Self {
            Self::Build(value)
        }
    }

    ///`AppType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "apps"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppType {
        #[serde(rename = "apps")]
        Apps,
    }

    impl ::std::fmt::Display for AppType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Apps => f.write_str("apps"),
            }
        }
    }

    impl ::std::str::FromStr for AppType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "apps" => Ok(Self::Apps),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppUpdateRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppUpdateRequest",
    ///  "type": "object",
    ///  "required": [
    ///    "data"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "attributes": {
    ///          "type": "object",
    ///          "properties": {
    ///            "accessibilityUrl": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ],
    ///              "format": "uri"
    ///            },
    ///            "bundleId": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "contentRightsDeclaration": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ],
    ///              "enum": [
    ///                "DOES_NOT_USE_THIRD_PARTY_CONTENT",
    ///                "USES_THIRD_PARTY_CONTENT"
    ///              ]
    ///            },
    ///            "primaryLocale": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "streamlinedPurchasingEnabled": {
    ///              "type": [
    ///                "boolean",
    ///                "null"
    ///              ]
    ///            },
    ///            "subscriptionStatusUrl": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ],
    ///              "format": "uri"
    ///            },
    ///            "subscriptionStatusUrlForSandbox": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ],
    ///              "format": "uri"
    ///            },
    ///            "subscriptionStatusUrlVersion": {
    ///              "$ref": "#/components/schemas/SubscriptionStatusUrlVersion"
    ///            },
    ///            "subscriptionStatusUrlVersionForSandbox": {
    ///              "$ref": "#/components/schemas/SubscriptionStatusUrlVersion"
    ///            }
    ///          }
    ///        },
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "apps"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppUpdateRequest {
        pub data: AppUpdateRequestData,
    }

    ///`AppUpdateRequestData`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "accessibilityUrl": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "uri"
    ///        },
    ///        "bundleId": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "contentRightsDeclaration": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "enum": [
    ///            "DOES_NOT_USE_THIRD_PARTY_CONTENT",
    ///            "USES_THIRD_PARTY_CONTENT"
    ///          ]
    ///        },
    ///        "primaryLocale": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "streamlinedPurchasingEnabled": {
    ///          "type": [
    ///            "boolean",
    ///            "null"
    ///          ]
    ///        },
    ///        "subscriptionStatusUrl": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "uri"
    ///        },
    ///        "subscriptionStatusUrlForSandbox": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "uri"
    ///        },
    ///        "subscriptionStatusUrlVersion": {
    ///          "$ref": "#/components/schemas/SubscriptionStatusUrlVersion"
    ///        },
    ///        "subscriptionStatusUrlVersionForSandbox": {
    ///          "$ref": "#/components/schemas/SubscriptionStatusUrlVersion"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "apps"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppUpdateRequestData {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppUpdateRequestDataAttributes>,
        pub id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: AppUpdateRequestDataType,
    }

    ///`AppUpdateRequestDataAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accessibilityUrl": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uri"
    ///    },
    ///    "bundleId": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "contentRightsDeclaration": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "enum": [
    ///        "DOES_NOT_USE_THIRD_PARTY_CONTENT",
    ///        "USES_THIRD_PARTY_CONTENT"
    ///      ]
    ///    },
    ///    "primaryLocale": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "streamlinedPurchasingEnabled": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "subscriptionStatusUrl": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uri"
    ///    },
    ///    "subscriptionStatusUrlForSandbox": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uri"
    ///    },
    ///    "subscriptionStatusUrlVersion": {
    ///      "$ref": "#/components/schemas/SubscriptionStatusUrlVersion"
    ///    },
    ///    "subscriptionStatusUrlVersionForSandbox": {
    ///      "$ref": "#/components/schemas/SubscriptionStatusUrlVersion"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppUpdateRequestDataAttributes {
        #[serde(
            rename = "accessibilityUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub accessibility_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "bundleId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bundle_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "contentRightsDeclaration",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub content_rights_declaration:
            ::std::option::Option<AppUpdateRequestDataAttributesContentRightsDeclaration>,
        #[serde(
            rename = "primaryLocale",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub primary_locale: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "streamlinedPurchasingEnabled",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub streamlined_purchasing_enabled: ::std::option::Option<bool>,
        #[serde(
            rename = "subscriptionStatusUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subscription_status_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "subscriptionStatusUrlForSandbox",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subscription_status_url_for_sandbox: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "subscriptionStatusUrlVersion",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subscription_status_url_version: ::std::option::Option<SubscriptionStatusUrlVersion>,
        #[serde(
            rename = "subscriptionStatusUrlVersionForSandbox",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subscription_status_url_version_for_sandbox:
            ::std::option::Option<SubscriptionStatusUrlVersion>,
    }

    impl ::std::default::Default for AppUpdateRequestDataAttributes {
        fn default() -> Self {
            Self {
                accessibility_url: Default::default(),
                bundle_id: Default::default(),
                content_rights_declaration: Default::default(),
                primary_locale: Default::default(),
                streamlined_purchasing_enabled: Default::default(),
                subscription_status_url: Default::default(),
                subscription_status_url_for_sandbox: Default::default(),
                subscription_status_url_version: Default::default(),
                subscription_status_url_version_for_sandbox: Default::default(),
            }
        }
    }

    ///`AppUpdateRequestDataAttributesContentRightsDeclaration`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "DOES_NOT_USE_THIRD_PARTY_CONTENT",
    ///    "USES_THIRD_PARTY_CONTENT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppUpdateRequestDataAttributesContentRightsDeclaration {
        #[serde(rename = "DOES_NOT_USE_THIRD_PARTY_CONTENT")]
        DoesNotUseThirdPartyContent,
        #[serde(rename = "USES_THIRD_PARTY_CONTENT")]
        UsesThirdPartyContent,
    }

    impl ::std::fmt::Display for AppUpdateRequestDataAttributesContentRightsDeclaration {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::DoesNotUseThirdPartyContent => {
                    f.write_str("DOES_NOT_USE_THIRD_PARTY_CONTENT")
                }
                Self::UsesThirdPartyContent => f.write_str("USES_THIRD_PARTY_CONTENT"),
            }
        }
    }

    impl ::std::str::FromStr for AppUpdateRequestDataAttributesContentRightsDeclaration {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "DOES_NOT_USE_THIRD_PARTY_CONTENT" => Ok(Self::DoesNotUseThirdPartyContent),
                "USES_THIRD_PARTY_CONTENT" => Ok(Self::UsesThirdPartyContent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppUpdateRequestDataAttributesContentRightsDeclaration {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppUpdateRequestDataAttributesContentRightsDeclaration
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppUpdateRequestDataAttributesContentRightsDeclaration
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppUpdateRequestDataType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "apps"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppUpdateRequestDataType {
        #[serde(rename = "apps")]
        Apps,
    }

    impl ::std::fmt::Display for AppUpdateRequestDataType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Apps => f.write_str("apps"),
            }
        }
    }

    impl ::std::str::FromStr for AppUpdateRequestDataType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "apps" => Ok(Self::Apps),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppUpdateRequestDataType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppUpdateRequestDataType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppUpdateRequestDataType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppInfosGetToManyRelatedFieldsAgeRatingDeclarationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "advertising",
    ///    "alcoholTobaccoOrDrugUseOrReferences",
    ///    "contests",
    ///    "gambling",
    ///    "gamblingSimulated",
    ///    "gunsOrOtherWeapons",
    ///    "healthOrWellnessTopics",
    ///    "kidsAgeBand",
    ///    "lootBox",
    ///    "medicalOrTreatmentInformation",
    ///    "messagingAndChat",
    ///    "parentalControls",
    ///    "profanityOrCrudeHumor",
    ///    "ageAssurance",
    ///    "sexualContentGraphicAndNudity",
    ///    "sexualContentOrNudity",
    ///    "horrorOrFearThemes",
    ///    "matureOrSuggestiveThemes",
    ///    "unrestrictedWebAccess",
    ///    "userGeneratedContent",
    ///    "violenceCartoonOrFantasy",
    ///    "violenceRealisticProlongedGraphicOrSadistic",
    ///    "violenceRealistic",
    ///    "ageRatingOverride",
    ///    "ageRatingOverrideV2",
    ///    "koreaAgeRatingOverride",
    ///    "developerAgeRatingInfoUrl"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppInfosGetToManyRelatedFieldsAgeRatingDeclarationsItem {
        #[serde(rename = "advertising")]
        Advertising,
        #[serde(rename = "alcoholTobaccoOrDrugUseOrReferences")]
        AlcoholTobaccoOrDrugUseOrReferences,
        #[serde(rename = "contests")]
        Contests,
        #[serde(rename = "gambling")]
        Gambling,
        #[serde(rename = "gamblingSimulated")]
        GamblingSimulated,
        #[serde(rename = "gunsOrOtherWeapons")]
        GunsOrOtherWeapons,
        #[serde(rename = "healthOrWellnessTopics")]
        HealthOrWellnessTopics,
        #[serde(rename = "kidsAgeBand")]
        KidsAgeBand,
        #[serde(rename = "lootBox")]
        LootBox,
        #[serde(rename = "medicalOrTreatmentInformation")]
        MedicalOrTreatmentInformation,
        #[serde(rename = "messagingAndChat")]
        MessagingAndChat,
        #[serde(rename = "parentalControls")]
        ParentalControls,
        #[serde(rename = "profanityOrCrudeHumor")]
        ProfanityOrCrudeHumor,
        #[serde(rename = "ageAssurance")]
        AgeAssurance,
        #[serde(rename = "sexualContentGraphicAndNudity")]
        SexualContentGraphicAndNudity,
        #[serde(rename = "sexualContentOrNudity")]
        SexualContentOrNudity,
        #[serde(rename = "horrorOrFearThemes")]
        HorrorOrFearThemes,
        #[serde(rename = "matureOrSuggestiveThemes")]
        MatureOrSuggestiveThemes,
        #[serde(rename = "unrestrictedWebAccess")]
        UnrestrictedWebAccess,
        #[serde(rename = "userGeneratedContent")]
        UserGeneratedContent,
        #[serde(rename = "violenceCartoonOrFantasy")]
        ViolenceCartoonOrFantasy,
        #[serde(rename = "violenceRealisticProlongedGraphicOrSadistic")]
        ViolenceRealisticProlongedGraphicOrSadistic,
        #[serde(rename = "violenceRealistic")]
        ViolenceRealistic,
        #[serde(rename = "ageRatingOverride")]
        AgeRatingOverride,
        #[serde(rename = "ageRatingOverrideV2")]
        AgeRatingOverrideV2,
        #[serde(rename = "koreaAgeRatingOverride")]
        KoreaAgeRatingOverride,
        #[serde(rename = "developerAgeRatingInfoUrl")]
        DeveloperAgeRatingInfoUrl,
    }

    impl ::std::fmt::Display for AppsAppInfosGetToManyRelatedFieldsAgeRatingDeclarationsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Advertising => f.write_str("advertising"),
                Self::AlcoholTobaccoOrDrugUseOrReferences => {
                    f.write_str("alcoholTobaccoOrDrugUseOrReferences")
                }
                Self::Contests => f.write_str("contests"),
                Self::Gambling => f.write_str("gambling"),
                Self::GamblingSimulated => f.write_str("gamblingSimulated"),
                Self::GunsOrOtherWeapons => f.write_str("gunsOrOtherWeapons"),
                Self::HealthOrWellnessTopics => f.write_str("healthOrWellnessTopics"),
                Self::KidsAgeBand => f.write_str("kidsAgeBand"),
                Self::LootBox => f.write_str("lootBox"),
                Self::MedicalOrTreatmentInformation => f.write_str("medicalOrTreatmentInformation"),
                Self::MessagingAndChat => f.write_str("messagingAndChat"),
                Self::ParentalControls => f.write_str("parentalControls"),
                Self::ProfanityOrCrudeHumor => f.write_str("profanityOrCrudeHumor"),
                Self::AgeAssurance => f.write_str("ageAssurance"),
                Self::SexualContentGraphicAndNudity => f.write_str("sexualContentGraphicAndNudity"),
                Self::SexualContentOrNudity => f.write_str("sexualContentOrNudity"),
                Self::HorrorOrFearThemes => f.write_str("horrorOrFearThemes"),
                Self::MatureOrSuggestiveThemes => f.write_str("matureOrSuggestiveThemes"),
                Self::UnrestrictedWebAccess => f.write_str("unrestrictedWebAccess"),
                Self::UserGeneratedContent => f.write_str("userGeneratedContent"),
                Self::ViolenceCartoonOrFantasy => f.write_str("violenceCartoonOrFantasy"),
                Self::ViolenceRealisticProlongedGraphicOrSadistic => {
                    f.write_str("violenceRealisticProlongedGraphicOrSadistic")
                }
                Self::ViolenceRealistic => f.write_str("violenceRealistic"),
                Self::AgeRatingOverride => f.write_str("ageRatingOverride"),
                Self::AgeRatingOverrideV2 => f.write_str("ageRatingOverrideV2"),
                Self::KoreaAgeRatingOverride => f.write_str("koreaAgeRatingOverride"),
                Self::DeveloperAgeRatingInfoUrl => f.write_str("developerAgeRatingInfoUrl"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppInfosGetToManyRelatedFieldsAgeRatingDeclarationsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "advertising" => Ok(Self::Advertising),
                "alcoholTobaccoOrDrugUseOrReferences" => {
                    Ok(Self::AlcoholTobaccoOrDrugUseOrReferences)
                }
                "contests" => Ok(Self::Contests),
                "gambling" => Ok(Self::Gambling),
                "gamblingSimulated" => Ok(Self::GamblingSimulated),
                "gunsOrOtherWeapons" => Ok(Self::GunsOrOtherWeapons),
                "healthOrWellnessTopics" => Ok(Self::HealthOrWellnessTopics),
                "kidsAgeBand" => Ok(Self::KidsAgeBand),
                "lootBox" => Ok(Self::LootBox),
                "medicalOrTreatmentInformation" => Ok(Self::MedicalOrTreatmentInformation),
                "messagingAndChat" => Ok(Self::MessagingAndChat),
                "parentalControls" => Ok(Self::ParentalControls),
                "profanityOrCrudeHumor" => Ok(Self::ProfanityOrCrudeHumor),
                "ageAssurance" => Ok(Self::AgeAssurance),
                "sexualContentGraphicAndNudity" => Ok(Self::SexualContentGraphicAndNudity),
                "sexualContentOrNudity" => Ok(Self::SexualContentOrNudity),
                "horrorOrFearThemes" => Ok(Self::HorrorOrFearThemes),
                "matureOrSuggestiveThemes" => Ok(Self::MatureOrSuggestiveThemes),
                "unrestrictedWebAccess" => Ok(Self::UnrestrictedWebAccess),
                "userGeneratedContent" => Ok(Self::UserGeneratedContent),
                "violenceCartoonOrFantasy" => Ok(Self::ViolenceCartoonOrFantasy),
                "violenceRealisticProlongedGraphicOrSadistic" => {
                    Ok(Self::ViolenceRealisticProlongedGraphicOrSadistic)
                }
                "violenceRealistic" => Ok(Self::ViolenceRealistic),
                "ageRatingOverride" => Ok(Self::AgeRatingOverride),
                "ageRatingOverrideV2" => Ok(Self::AgeRatingOverrideV2),
                "koreaAgeRatingOverride" => Ok(Self::KoreaAgeRatingOverride),
                "developerAgeRatingInfoUrl" => Ok(Self::DeveloperAgeRatingInfoUrl),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppInfosGetToManyRelatedFieldsAgeRatingDeclarationsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppInfosGetToManyRelatedFieldsAgeRatingDeclarationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppInfosGetToManyRelatedFieldsAgeRatingDeclarationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppInfosGetToManyRelatedFieldsAppCategoriesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platforms",
    ///    "subcategories",
    ///    "parent"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppInfosGetToManyRelatedFieldsAppCategoriesItem {
        #[serde(rename = "platforms")]
        Platforms,
        #[serde(rename = "subcategories")]
        Subcategories,
        #[serde(rename = "parent")]
        Parent,
    }

    impl ::std::fmt::Display for AppsAppInfosGetToManyRelatedFieldsAppCategoriesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platforms => f.write_str("platforms"),
                Self::Subcategories => f.write_str("subcategories"),
                Self::Parent => f.write_str("parent"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppInfosGetToManyRelatedFieldsAppCategoriesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platforms" => Ok(Self::Platforms),
                "subcategories" => Ok(Self::Subcategories),
                "parent" => Ok(Self::Parent),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppInfosGetToManyRelatedFieldsAppCategoriesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppInfosGetToManyRelatedFieldsAppCategoriesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppInfosGetToManyRelatedFieldsAppCategoriesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppInfosGetToManyRelatedFieldsAppInfoLocalizationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "locale",
    ///    "name",
    ///    "subtitle",
    ///    "privacyPolicyUrl",
    ///    "privacyChoicesUrl",
    ///    "privacyPolicyText",
    ///    "appInfo"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppInfosGetToManyRelatedFieldsAppInfoLocalizationsItem {
        #[serde(rename = "locale")]
        Locale,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "subtitle")]
        Subtitle,
        #[serde(rename = "privacyPolicyUrl")]
        PrivacyPolicyUrl,
        #[serde(rename = "privacyChoicesUrl")]
        PrivacyChoicesUrl,
        #[serde(rename = "privacyPolicyText")]
        PrivacyPolicyText,
        #[serde(rename = "appInfo")]
        AppInfo,
    }

    impl ::std::fmt::Display for AppsAppInfosGetToManyRelatedFieldsAppInfoLocalizationsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Locale => f.write_str("locale"),
                Self::Name => f.write_str("name"),
                Self::Subtitle => f.write_str("subtitle"),
                Self::PrivacyPolicyUrl => f.write_str("privacyPolicyUrl"),
                Self::PrivacyChoicesUrl => f.write_str("privacyChoicesUrl"),
                Self::PrivacyPolicyText => f.write_str("privacyPolicyText"),
                Self::AppInfo => f.write_str("appInfo"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppInfosGetToManyRelatedFieldsAppInfoLocalizationsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "locale" => Ok(Self::Locale),
                "name" => Ok(Self::Name),
                "subtitle" => Ok(Self::Subtitle),
                "privacyPolicyUrl" => Ok(Self::PrivacyPolicyUrl),
                "privacyChoicesUrl" => Ok(Self::PrivacyChoicesUrl),
                "privacyPolicyText" => Ok(Self::PrivacyPolicyText),
                "appInfo" => Ok(Self::AppInfo),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppInfosGetToManyRelatedFieldsAppInfoLocalizationsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppInfosGetToManyRelatedFieldsAppInfoLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppInfosGetToManyRelatedFieldsAppInfoLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppInfosGetToManyRelatedFieldsAppInfosItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreState",
    ///    "state",
    ///    "appStoreAgeRating",
    ///    "australiaAgeRating",
    ///    "brazilAgeRating",
    ///    "brazilAgeRatingV2",
    ///    "franceAgeRating",
    ///    "koreaAgeRating",
    ///    "app",
    ///    "ageRatingDeclaration",
    ///    "appInfoLocalizations",
    ///    "primaryCategory",
    ///    "primarySubcategoryOne",
    ///    "primarySubcategoryTwo",
    ///    "secondaryCategory",
    ///    "secondarySubcategoryOne",
    ///    "secondarySubcategoryTwo",
    ///    "territoryAgeRatings"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppInfosGetToManyRelatedFieldsAppInfosItem {
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "appStoreAgeRating")]
        AppStoreAgeRating,
        #[serde(rename = "australiaAgeRating")]
        AustraliaAgeRating,
        #[serde(rename = "brazilAgeRating")]
        BrazilAgeRating,
        #[serde(rename = "brazilAgeRatingV2")]
        BrazilAgeRatingV2,
        #[serde(rename = "franceAgeRating")]
        FranceAgeRating,
        #[serde(rename = "koreaAgeRating")]
        KoreaAgeRating,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "ageRatingDeclaration")]
        AgeRatingDeclaration,
        #[serde(rename = "appInfoLocalizations")]
        AppInfoLocalizations,
        #[serde(rename = "primaryCategory")]
        PrimaryCategory,
        #[serde(rename = "primarySubcategoryOne")]
        PrimarySubcategoryOne,
        #[serde(rename = "primarySubcategoryTwo")]
        PrimarySubcategoryTwo,
        #[serde(rename = "secondaryCategory")]
        SecondaryCategory,
        #[serde(rename = "secondarySubcategoryOne")]
        SecondarySubcategoryOne,
        #[serde(rename = "secondarySubcategoryTwo")]
        SecondarySubcategoryTwo,
        #[serde(rename = "territoryAgeRatings")]
        TerritoryAgeRatings,
    }

    impl ::std::fmt::Display for AppsAppInfosGetToManyRelatedFieldsAppInfosItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::State => f.write_str("state"),
                Self::AppStoreAgeRating => f.write_str("appStoreAgeRating"),
                Self::AustraliaAgeRating => f.write_str("australiaAgeRating"),
                Self::BrazilAgeRating => f.write_str("brazilAgeRating"),
                Self::BrazilAgeRatingV2 => f.write_str("brazilAgeRatingV2"),
                Self::FranceAgeRating => f.write_str("franceAgeRating"),
                Self::KoreaAgeRating => f.write_str("koreaAgeRating"),
                Self::App => f.write_str("app"),
                Self::AgeRatingDeclaration => f.write_str("ageRatingDeclaration"),
                Self::AppInfoLocalizations => f.write_str("appInfoLocalizations"),
                Self::PrimaryCategory => f.write_str("primaryCategory"),
                Self::PrimarySubcategoryOne => f.write_str("primarySubcategoryOne"),
                Self::PrimarySubcategoryTwo => f.write_str("primarySubcategoryTwo"),
                Self::SecondaryCategory => f.write_str("secondaryCategory"),
                Self::SecondarySubcategoryOne => f.write_str("secondarySubcategoryOne"),
                Self::SecondarySubcategoryTwo => f.write_str("secondarySubcategoryTwo"),
                Self::TerritoryAgeRatings => f.write_str("territoryAgeRatings"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppInfosGetToManyRelatedFieldsAppInfosItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreState" => Ok(Self::AppStoreState),
                "state" => Ok(Self::State),
                "appStoreAgeRating" => Ok(Self::AppStoreAgeRating),
                "australiaAgeRating" => Ok(Self::AustraliaAgeRating),
                "brazilAgeRating" => Ok(Self::BrazilAgeRating),
                "brazilAgeRatingV2" => Ok(Self::BrazilAgeRatingV2),
                "franceAgeRating" => Ok(Self::FranceAgeRating),
                "koreaAgeRating" => Ok(Self::KoreaAgeRating),
                "app" => Ok(Self::App),
                "ageRatingDeclaration" => Ok(Self::AgeRatingDeclaration),
                "appInfoLocalizations" => Ok(Self::AppInfoLocalizations),
                "primaryCategory" => Ok(Self::PrimaryCategory),
                "primarySubcategoryOne" => Ok(Self::PrimarySubcategoryOne),
                "primarySubcategoryTwo" => Ok(Self::PrimarySubcategoryTwo),
                "secondaryCategory" => Ok(Self::SecondaryCategory),
                "secondarySubcategoryOne" => Ok(Self::SecondarySubcategoryOne),
                "secondarySubcategoryTwo" => Ok(Self::SecondarySubcategoryTwo),
                "territoryAgeRatings" => Ok(Self::TerritoryAgeRatings),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppInfosGetToManyRelatedFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppInfosGetToManyRelatedFieldsAppInfosItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppInfosGetToManyRelatedFieldsAppInfosItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppInfosGetToManyRelatedFieldsAppsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "accessibilityUrl",
    ///    "name",
    ///    "bundleId",
    ///    "sku",
    ///    "primaryLocale",
    ///    "isOrEverWasMadeForKids",
    ///    "subscriptionStatusUrl",
    ///    "subscriptionStatusUrlVersion",
    ///    "subscriptionStatusUrlForSandbox",
    ///    "subscriptionStatusUrlVersionForSandbox",
    ///    "contentRightsDeclaration",
    ///    "streamlinedPurchasingEnabled",
    ///    "accessibilityDeclarations",
    ///    "appEncryptionDeclarations",
    ///    "appStoreIcon",
    ///    "ciProduct",
    ///    "betaTesters",
    ///    "betaGroups",
    ///    "appStoreVersions",
    ///    "appTags",
    ///    "preReleaseVersions",
    ///    "betaAppLocalizations",
    ///    "builds",
    ///    "betaLicenseAgreement",
    ///    "betaAppReviewDetail",
    ///    "appInfos",
    ///    "appClips",
    ///    "appPricePoints",
    ///    "endUserLicenseAgreement",
    ///    "appPriceSchedule",
    ///    "appAvailabilityV2",
    ///    "inAppPurchases",
    ///    "subscriptionGroups",
    ///    "gameCenterEnabledVersions",
    ///    "perfPowerMetrics",
    ///    "appCustomProductPages",
    ///    "inAppPurchasesV2",
    ///    "promotedPurchases",
    ///    "appEvents",
    ///    "reviewSubmissions",
    ///    "subscriptionGracePeriod",
    ///    "customerReviews",
    ///    "customerReviewSummarizations",
    ///    "gameCenterDetail",
    ///    "appStoreVersionExperimentsV2",
    ///    "alternativeDistributionKey",
    ///    "analyticsReportRequests",
    ///    "marketplaceSearchDetail",
    ///    "buildUploads",
    ///    "backgroundAssets",
    ///    "betaFeedbackScreenshotSubmissions",
    ///    "betaFeedbackCrashSubmissions",
    ///    "searchKeywords",
    ///    "webhooks",
    ///    "androidToIosAppMappingDetails"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppInfosGetToManyRelatedFieldsAppsItem {
        #[serde(rename = "accessibilityUrl")]
        AccessibilityUrl,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "sku")]
        Sku,
        #[serde(rename = "primaryLocale")]
        PrimaryLocale,
        #[serde(rename = "isOrEverWasMadeForKids")]
        IsOrEverWasMadeForKids,
        #[serde(rename = "subscriptionStatusUrl")]
        SubscriptionStatusUrl,
        #[serde(rename = "subscriptionStatusUrlVersion")]
        SubscriptionStatusUrlVersion,
        #[serde(rename = "subscriptionStatusUrlForSandbox")]
        SubscriptionStatusUrlForSandbox,
        #[serde(rename = "subscriptionStatusUrlVersionForSandbox")]
        SubscriptionStatusUrlVersionForSandbox,
        #[serde(rename = "contentRightsDeclaration")]
        ContentRightsDeclaration,
        #[serde(rename = "streamlinedPurchasingEnabled")]
        StreamlinedPurchasingEnabled,
        #[serde(rename = "accessibilityDeclarations")]
        AccessibilityDeclarations,
        #[serde(rename = "appEncryptionDeclarations")]
        AppEncryptionDeclarations,
        #[serde(rename = "appStoreIcon")]
        AppStoreIcon,
        #[serde(rename = "ciProduct")]
        CiProduct,
        #[serde(rename = "betaTesters")]
        BetaTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
        #[serde(rename = "appTags")]
        AppTags,
        #[serde(rename = "preReleaseVersions")]
        PreReleaseVersions,
        #[serde(rename = "betaAppLocalizations")]
        BetaAppLocalizations,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaLicenseAgreement")]
        BetaLicenseAgreement,
        #[serde(rename = "betaAppReviewDetail")]
        BetaAppReviewDetail,
        #[serde(rename = "appInfos")]
        AppInfos,
        #[serde(rename = "appClips")]
        AppClips,
        #[serde(rename = "appPricePoints")]
        AppPricePoints,
        #[serde(rename = "endUserLicenseAgreement")]
        EndUserLicenseAgreement,
        #[serde(rename = "appPriceSchedule")]
        AppPriceSchedule,
        #[serde(rename = "appAvailabilityV2")]
        AppAvailabilityV2,
        #[serde(rename = "inAppPurchases")]
        InAppPurchases,
        #[serde(rename = "subscriptionGroups")]
        SubscriptionGroups,
        #[serde(rename = "gameCenterEnabledVersions")]
        GameCenterEnabledVersions,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "appCustomProductPages")]
        AppCustomProductPages,
        #[serde(rename = "inAppPurchasesV2")]
        InAppPurchasesV2,
        #[serde(rename = "promotedPurchases")]
        PromotedPurchases,
        #[serde(rename = "appEvents")]
        AppEvents,
        #[serde(rename = "reviewSubmissions")]
        ReviewSubmissions,
        #[serde(rename = "subscriptionGracePeriod")]
        SubscriptionGracePeriod,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "customerReviewSummarizations")]
        CustomerReviewSummarizations,
        #[serde(rename = "gameCenterDetail")]
        GameCenterDetail,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "alternativeDistributionKey")]
        AlternativeDistributionKey,
        #[serde(rename = "analyticsReportRequests")]
        AnalyticsReportRequests,
        #[serde(rename = "marketplaceSearchDetail")]
        MarketplaceSearchDetail,
        #[serde(rename = "buildUploads")]
        BuildUploads,
        #[serde(rename = "backgroundAssets")]
        BackgroundAssets,
        #[serde(rename = "betaFeedbackScreenshotSubmissions")]
        BetaFeedbackScreenshotSubmissions,
        #[serde(rename = "betaFeedbackCrashSubmissions")]
        BetaFeedbackCrashSubmissions,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
        #[serde(rename = "webhooks")]
        Webhooks,
        #[serde(rename = "androidToIosAppMappingDetails")]
        AndroidToIosAppMappingDetails,
    }

    impl ::std::fmt::Display for AppsAppInfosGetToManyRelatedFieldsAppsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AccessibilityUrl => f.write_str("accessibilityUrl"),
                Self::Name => f.write_str("name"),
                Self::BundleId => f.write_str("bundleId"),
                Self::Sku => f.write_str("sku"),
                Self::PrimaryLocale => f.write_str("primaryLocale"),
                Self::IsOrEverWasMadeForKids => f.write_str("isOrEverWasMadeForKids"),
                Self::SubscriptionStatusUrl => f.write_str("subscriptionStatusUrl"),
                Self::SubscriptionStatusUrlVersion => f.write_str("subscriptionStatusUrlVersion"),
                Self::SubscriptionStatusUrlForSandbox => {
                    f.write_str("subscriptionStatusUrlForSandbox")
                }
                Self::SubscriptionStatusUrlVersionForSandbox => {
                    f.write_str("subscriptionStatusUrlVersionForSandbox")
                }
                Self::ContentRightsDeclaration => f.write_str("contentRightsDeclaration"),
                Self::StreamlinedPurchasingEnabled => f.write_str("streamlinedPurchasingEnabled"),
                Self::AccessibilityDeclarations => f.write_str("accessibilityDeclarations"),
                Self::AppEncryptionDeclarations => f.write_str("appEncryptionDeclarations"),
                Self::AppStoreIcon => f.write_str("appStoreIcon"),
                Self::CiProduct => f.write_str("ciProduct"),
                Self::BetaTesters => f.write_str("betaTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
                Self::AppTags => f.write_str("appTags"),
                Self::PreReleaseVersions => f.write_str("preReleaseVersions"),
                Self::BetaAppLocalizations => f.write_str("betaAppLocalizations"),
                Self::Builds => f.write_str("builds"),
                Self::BetaLicenseAgreement => f.write_str("betaLicenseAgreement"),
                Self::BetaAppReviewDetail => f.write_str("betaAppReviewDetail"),
                Self::AppInfos => f.write_str("appInfos"),
                Self::AppClips => f.write_str("appClips"),
                Self::AppPricePoints => f.write_str("appPricePoints"),
                Self::EndUserLicenseAgreement => f.write_str("endUserLicenseAgreement"),
                Self::AppPriceSchedule => f.write_str("appPriceSchedule"),
                Self::AppAvailabilityV2 => f.write_str("appAvailabilityV2"),
                Self::InAppPurchases => f.write_str("inAppPurchases"),
                Self::SubscriptionGroups => f.write_str("subscriptionGroups"),
                Self::GameCenterEnabledVersions => f.write_str("gameCenterEnabledVersions"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::AppCustomProductPages => f.write_str("appCustomProductPages"),
                Self::InAppPurchasesV2 => f.write_str("inAppPurchasesV2"),
                Self::PromotedPurchases => f.write_str("promotedPurchases"),
                Self::AppEvents => f.write_str("appEvents"),
                Self::ReviewSubmissions => f.write_str("reviewSubmissions"),
                Self::SubscriptionGracePeriod => f.write_str("subscriptionGracePeriod"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::CustomerReviewSummarizations => f.write_str("customerReviewSummarizations"),
                Self::GameCenterDetail => f.write_str("gameCenterDetail"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AlternativeDistributionKey => f.write_str("alternativeDistributionKey"),
                Self::AnalyticsReportRequests => f.write_str("analyticsReportRequests"),
                Self::MarketplaceSearchDetail => f.write_str("marketplaceSearchDetail"),
                Self::BuildUploads => f.write_str("buildUploads"),
                Self::BackgroundAssets => f.write_str("backgroundAssets"),
                Self::BetaFeedbackScreenshotSubmissions => {
                    f.write_str("betaFeedbackScreenshotSubmissions")
                }
                Self::BetaFeedbackCrashSubmissions => f.write_str("betaFeedbackCrashSubmissions"),
                Self::SearchKeywords => f.write_str("searchKeywords"),
                Self::Webhooks => f.write_str("webhooks"),
                Self::AndroidToIosAppMappingDetails => f.write_str("androidToIosAppMappingDetails"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppInfosGetToManyRelatedFieldsAppsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "accessibilityUrl" => Ok(Self::AccessibilityUrl),
                "name" => Ok(Self::Name),
                "bundleId" => Ok(Self::BundleId),
                "sku" => Ok(Self::Sku),
                "primaryLocale" => Ok(Self::PrimaryLocale),
                "isOrEverWasMadeForKids" => Ok(Self::IsOrEverWasMadeForKids),
                "subscriptionStatusUrl" => Ok(Self::SubscriptionStatusUrl),
                "subscriptionStatusUrlVersion" => Ok(Self::SubscriptionStatusUrlVersion),
                "subscriptionStatusUrlForSandbox" => Ok(Self::SubscriptionStatusUrlForSandbox),
                "subscriptionStatusUrlVersionForSandbox" => {
                    Ok(Self::SubscriptionStatusUrlVersionForSandbox)
                }
                "contentRightsDeclaration" => Ok(Self::ContentRightsDeclaration),
                "streamlinedPurchasingEnabled" => Ok(Self::StreamlinedPurchasingEnabled),
                "accessibilityDeclarations" => Ok(Self::AccessibilityDeclarations),
                "appEncryptionDeclarations" => Ok(Self::AppEncryptionDeclarations),
                "appStoreIcon" => Ok(Self::AppStoreIcon),
                "ciProduct" => Ok(Self::CiProduct),
                "betaTesters" => Ok(Self::BetaTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                "appTags" => Ok(Self::AppTags),
                "preReleaseVersions" => Ok(Self::PreReleaseVersions),
                "betaAppLocalizations" => Ok(Self::BetaAppLocalizations),
                "builds" => Ok(Self::Builds),
                "betaLicenseAgreement" => Ok(Self::BetaLicenseAgreement),
                "betaAppReviewDetail" => Ok(Self::BetaAppReviewDetail),
                "appInfos" => Ok(Self::AppInfos),
                "appClips" => Ok(Self::AppClips),
                "appPricePoints" => Ok(Self::AppPricePoints),
                "endUserLicenseAgreement" => Ok(Self::EndUserLicenseAgreement),
                "appPriceSchedule" => Ok(Self::AppPriceSchedule),
                "appAvailabilityV2" => Ok(Self::AppAvailabilityV2),
                "inAppPurchases" => Ok(Self::InAppPurchases),
                "subscriptionGroups" => Ok(Self::SubscriptionGroups),
                "gameCenterEnabledVersions" => Ok(Self::GameCenterEnabledVersions),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "appCustomProductPages" => Ok(Self::AppCustomProductPages),
                "inAppPurchasesV2" => Ok(Self::InAppPurchasesV2),
                "promotedPurchases" => Ok(Self::PromotedPurchases),
                "appEvents" => Ok(Self::AppEvents),
                "reviewSubmissions" => Ok(Self::ReviewSubmissions),
                "subscriptionGracePeriod" => Ok(Self::SubscriptionGracePeriod),
                "customerReviews" => Ok(Self::CustomerReviews),
                "customerReviewSummarizations" => Ok(Self::CustomerReviewSummarizations),
                "gameCenterDetail" => Ok(Self::GameCenterDetail),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "alternativeDistributionKey" => Ok(Self::AlternativeDistributionKey),
                "analyticsReportRequests" => Ok(Self::AnalyticsReportRequests),
                "marketplaceSearchDetail" => Ok(Self::MarketplaceSearchDetail),
                "buildUploads" => Ok(Self::BuildUploads),
                "backgroundAssets" => Ok(Self::BackgroundAssets),
                "betaFeedbackScreenshotSubmissions" => Ok(Self::BetaFeedbackScreenshotSubmissions),
                "betaFeedbackCrashSubmissions" => Ok(Self::BetaFeedbackCrashSubmissions),
                "searchKeywords" => Ok(Self::SearchKeywords),
                "webhooks" => Ok(Self::Webhooks),
                "androidToIosAppMappingDetails" => Ok(Self::AndroidToIosAppMappingDetails),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppInfosGetToManyRelatedFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppInfosGetToManyRelatedFieldsAppsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsAppInfosGetToManyRelatedFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppInfosGetToManyRelatedIncludeItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "app",
    ///    "ageRatingDeclaration",
    ///    "appInfoLocalizations",
    ///    "primaryCategory",
    ///    "primarySubcategoryOne",
    ///    "primarySubcategoryTwo",
    ///    "secondaryCategory",
    ///    "secondarySubcategoryOne",
    ///    "secondarySubcategoryTwo"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppInfosGetToManyRelatedIncludeItem {
        #[serde(rename = "app")]
        App,
        #[serde(rename = "ageRatingDeclaration")]
        AgeRatingDeclaration,
        #[serde(rename = "appInfoLocalizations")]
        AppInfoLocalizations,
        #[serde(rename = "primaryCategory")]
        PrimaryCategory,
        #[serde(rename = "primarySubcategoryOne")]
        PrimarySubcategoryOne,
        #[serde(rename = "primarySubcategoryTwo")]
        PrimarySubcategoryTwo,
        #[serde(rename = "secondaryCategory")]
        SecondaryCategory,
        #[serde(rename = "secondarySubcategoryOne")]
        SecondarySubcategoryOne,
        #[serde(rename = "secondarySubcategoryTwo")]
        SecondarySubcategoryTwo,
    }

    impl ::std::fmt::Display for AppsAppInfosGetToManyRelatedIncludeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::App => f.write_str("app"),
                Self::AgeRatingDeclaration => f.write_str("ageRatingDeclaration"),
                Self::AppInfoLocalizations => f.write_str("appInfoLocalizations"),
                Self::PrimaryCategory => f.write_str("primaryCategory"),
                Self::PrimarySubcategoryOne => f.write_str("primarySubcategoryOne"),
                Self::PrimarySubcategoryTwo => f.write_str("primarySubcategoryTwo"),
                Self::SecondaryCategory => f.write_str("secondaryCategory"),
                Self::SecondarySubcategoryOne => f.write_str("secondarySubcategoryOne"),
                Self::SecondarySubcategoryTwo => f.write_str("secondarySubcategoryTwo"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppInfosGetToManyRelatedIncludeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "app" => Ok(Self::App),
                "ageRatingDeclaration" => Ok(Self::AgeRatingDeclaration),
                "appInfoLocalizations" => Ok(Self::AppInfoLocalizations),
                "primaryCategory" => Ok(Self::PrimaryCategory),
                "primarySubcategoryOne" => Ok(Self::PrimarySubcategoryOne),
                "primarySubcategoryTwo" => Ok(Self::PrimarySubcategoryTwo),
                "secondaryCategory" => Ok(Self::SecondaryCategory),
                "secondarySubcategoryOne" => Ok(Self::SecondarySubcategoryOne),
                "secondarySubcategoryTwo" => Ok(Self::SecondarySubcategoryTwo),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppInfosGetToManyRelatedIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsAppInfosGetToManyRelatedIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsAppInfosGetToManyRelatedIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "sourceFileChecksum",
    ///    "versions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem {
        #[serde(rename = "sourceFileChecksum")]
        SourceFileChecksum,
        #[serde(rename = "versions")]
        Versions,
    }

    impl ::std::fmt::Display
        for AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SourceFileChecksum => f.write_str("sourceFileChecksum"),
                Self::Versions => f.write_str("versions"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "sourceFileChecksum" => Ok(Self::SourceFileChecksum),
                "versions" => Ok(Self::Versions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "action",
    ///    "appClip",
    ///    "releaseWithAppStoreVersion",
    ///    "appClipDefaultExperienceLocalizations",
    ///    "appClipAppStoreReviewDetail"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem {
        #[serde(rename = "action")]
        Action,
        #[serde(rename = "appClip")]
        AppClip,
        #[serde(rename = "releaseWithAppStoreVersion")]
        ReleaseWithAppStoreVersion,
        #[serde(rename = "appClipDefaultExperienceLocalizations")]
        AppClipDefaultExperienceLocalizations,
        #[serde(rename = "appClipAppStoreReviewDetail")]
        AppClipAppStoreReviewDetail,
    }

    impl ::std::fmt::Display
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Action => f.write_str("action"),
                Self::AppClip => f.write_str("appClip"),
                Self::ReleaseWithAppStoreVersion => f.write_str("releaseWithAppStoreVersion"),
                Self::AppClipDefaultExperienceLocalizations => {
                    f.write_str("appClipDefaultExperienceLocalizations")
                }
                Self::AppClipAppStoreReviewDetail => f.write_str("appClipAppStoreReviewDetail"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "action" => Ok(Self::Action),
                "appClip" => Ok(Self::AppClip),
                "releaseWithAppStoreVersion" => Ok(Self::ReleaseWithAppStoreVersion),
                "appClipDefaultExperienceLocalizations" => {
                    Ok(Self::AppClipDefaultExperienceLocalizations)
                }
                "appClipAppStoreReviewDetail" => Ok(Self::AppClipAppStoreReviewDetail),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "contactFirstName",
    ///    "contactLastName",
    ///    "contactPhone",
    ///    "contactEmail",
    ///    "demoAccountName",
    ///    "demoAccountPassword",
    ///    "demoAccountRequired",
    ///    "notes",
    ///    "appStoreVersion",
    ///    "appStoreReviewAttachments"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem {
        #[serde(rename = "contactFirstName")]
        ContactFirstName,
        #[serde(rename = "contactLastName")]
        ContactLastName,
        #[serde(rename = "contactPhone")]
        ContactPhone,
        #[serde(rename = "contactEmail")]
        ContactEmail,
        #[serde(rename = "demoAccountName")]
        DemoAccountName,
        #[serde(rename = "demoAccountPassword")]
        DemoAccountPassword,
        #[serde(rename = "demoAccountRequired")]
        DemoAccountRequired,
        #[serde(rename = "notes")]
        Notes,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "appStoreReviewAttachments")]
        AppStoreReviewAttachments,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ContactFirstName => f.write_str("contactFirstName"),
                Self::ContactLastName => f.write_str("contactLastName"),
                Self::ContactPhone => f.write_str("contactPhone"),
                Self::ContactEmail => f.write_str("contactEmail"),
                Self::DemoAccountName => f.write_str("demoAccountName"),
                Self::DemoAccountPassword => f.write_str("demoAccountPassword"),
                Self::DemoAccountRequired => f.write_str("demoAccountRequired"),
                Self::Notes => f.write_str("notes"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::AppStoreReviewAttachments => f.write_str("appStoreReviewAttachments"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "contactFirstName" => Ok(Self::ContactFirstName),
                "contactLastName" => Ok(Self::ContactLastName),
                "contactPhone" => Ok(Self::ContactPhone),
                "contactEmail" => Ok(Self::ContactEmail),
                "demoAccountName" => Ok(Self::DemoAccountName),
                "demoAccountPassword" => Ok(Self::DemoAccountPassword),
                "demoAccountRequired" => Ok(Self::DemoAccountRequired),
                "notes" => Ok(Self::Notes),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "appStoreReviewAttachments" => Ok(Self::AppStoreReviewAttachments),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "trafficProportion",
    ///    "state",
    ///    "reviewRequired",
    ///    "startDate",
    ///    "endDate",
    ///    "appStoreVersion",
    ///    "appStoreVersionExperimentTreatments",
    ///    "platform",
    ///    "app",
    ///    "latestControlVersion",
    ///    "controlVersions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "trafficProportion")]
        TrafficProportion,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "reviewRequired")]
        ReviewRequired,
        #[serde(rename = "startDate")]
        StartDate,
        #[serde(rename = "endDate")]
        EndDate,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "appStoreVersionExperimentTreatments")]
        AppStoreVersionExperimentTreatments,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "latestControlVersion")]
        LatestControlVersion,
        #[serde(rename = "controlVersions")]
        ControlVersions,
    }

    impl ::std::fmt::Display
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::TrafficProportion => f.write_str("trafficProportion"),
                Self::State => f.write_str("state"),
                Self::ReviewRequired => f.write_str("reviewRequired"),
                Self::StartDate => f.write_str("startDate"),
                Self::EndDate => f.write_str("endDate"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::AppStoreVersionExperimentTreatments => {
                    f.write_str("appStoreVersionExperimentTreatments")
                }
                Self::Platform => f.write_str("platform"),
                Self::App => f.write_str("app"),
                Self::LatestControlVersion => f.write_str("latestControlVersion"),
                Self::ControlVersions => f.write_str("controlVersions"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "trafficProportion" => Ok(Self::TrafficProportion),
                "state" => Ok(Self::State),
                "reviewRequired" => Ok(Self::ReviewRequired),
                "startDate" => Ok(Self::StartDate),
                "endDate" => Ok(Self::EndDate),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "appStoreVersionExperimentTreatments" => {
                    Ok(Self::AppStoreVersionExperimentTreatments)
                }
                "platform" => Ok(Self::Platform),
                "app" => Ok(Self::App),
                "latestControlVersion" => Ok(Self::LatestControlVersion),
                "controlVersions" => Ok(Self::ControlVersions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "description",
    ///    "locale",
    ///    "keywords",
    ///    "marketingUrl",
    ///    "promotionalText",
    ///    "supportUrl",
    ///    "whatsNew",
    ///    "appStoreVersion",
    ///    "appScreenshotSets",
    ///    "appPreviewSets",
    ///    "searchKeywords"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem {
        #[serde(rename = "description")]
        Description,
        #[serde(rename = "locale")]
        Locale,
        #[serde(rename = "keywords")]
        Keywords,
        #[serde(rename = "marketingUrl")]
        MarketingUrl,
        #[serde(rename = "promotionalText")]
        PromotionalText,
        #[serde(rename = "supportUrl")]
        SupportUrl,
        #[serde(rename = "whatsNew")]
        WhatsNew,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "appScreenshotSets")]
        AppScreenshotSets,
        #[serde(rename = "appPreviewSets")]
        AppPreviewSets,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
    }

    impl ::std::fmt::Display
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Description => f.write_str("description"),
                Self::Locale => f.write_str("locale"),
                Self::Keywords => f.write_str("keywords"),
                Self::MarketingUrl => f.write_str("marketingUrl"),
                Self::PromotionalText => f.write_str("promotionalText"),
                Self::SupportUrl => f.write_str("supportUrl"),
                Self::WhatsNew => f.write_str("whatsNew"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::AppScreenshotSets => f.write_str("appScreenshotSets"),
                Self::AppPreviewSets => f.write_str("appPreviewSets"),
                Self::SearchKeywords => f.write_str("searchKeywords"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "description" => Ok(Self::Description),
                "locale" => Ok(Self::Locale),
                "keywords" => Ok(Self::Keywords),
                "marketingUrl" => Ok(Self::MarketingUrl),
                "promotionalText" => Ok(Self::PromotionalText),
                "supportUrl" => Ok(Self::SupportUrl),
                "whatsNew" => Ok(Self::WhatsNew),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "appScreenshotSets" => Ok(Self::AppScreenshotSets),
                "appPreviewSets" => Ok(Self::AppPreviewSets),
                "searchKeywords" => Ok(Self::SearchKeywords),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "phasedReleaseState",
    ///    "startDate",
    ///    "totalPauseDuration",
    ///    "currentDayNumber"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem {
        #[serde(rename = "phasedReleaseState")]
        PhasedReleaseState,
        #[serde(rename = "startDate")]
        StartDate,
        #[serde(rename = "totalPauseDuration")]
        TotalPauseDuration,
        #[serde(rename = "currentDayNumber")]
        CurrentDayNumber,
    }

    impl ::std::fmt::Display
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::PhasedReleaseState => f.write_str("phasedReleaseState"),
                Self::StartDate => f.write_str("startDate"),
                Self::TotalPauseDuration => f.write_str("totalPauseDuration"),
                Self::CurrentDayNumber => f.write_str("currentDayNumber"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "phasedReleaseState" => Ok(Self::PhasedReleaseState),
                "startDate" => Ok(Self::StartDate),
                "totalPauseDuration" => Ok(Self::TotalPauseDuration),
                "currentDayNumber" => Ok(Self::CurrentDayNumber),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersion"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem {
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
    }

    impl ::std::fmt::Display
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "versionString",
    ///    "appStoreState",
    ///    "appVersionState",
    ///    "copyright",
    ///    "reviewType",
    ///    "releaseType",
    ///    "earliestReleaseDate",
    ///    "usesIdfa",
    ///    "downloadable",
    ///    "createdDate",
    ///    "app",
    ///    "appStoreVersionLocalizations",
    ///    "build",
    ///    "appStoreVersionPhasedRelease",
    ///    "gameCenterAppVersion",
    ///    "routingAppCoverage",
    ///    "appStoreReviewDetail",
    ///    "appStoreVersionSubmission",
    ///    "appClipDefaultExperience",
    ///    "appStoreVersionExperiments",
    ///    "appStoreVersionExperimentsV2",
    ///    "customerReviews",
    ///    "alternativeDistributionPackage"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "versionString")]
        VersionString,
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "appVersionState")]
        AppVersionState,
        #[serde(rename = "copyright")]
        Copyright,
        #[serde(rename = "reviewType")]
        ReviewType,
        #[serde(rename = "releaseType")]
        ReleaseType,
        #[serde(rename = "earliestReleaseDate")]
        EarliestReleaseDate,
        #[serde(rename = "usesIdfa")]
        UsesIdfa,
        #[serde(rename = "downloadable")]
        Downloadable,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "appStoreVersionPhasedRelease")]
        AppStoreVersionPhasedRelease,
        #[serde(rename = "gameCenterAppVersion")]
        GameCenterAppVersion,
        #[serde(rename = "routingAppCoverage")]
        RoutingAppCoverage,
        #[serde(rename = "appStoreReviewDetail")]
        AppStoreReviewDetail,
        #[serde(rename = "appStoreVersionSubmission")]
        AppStoreVersionSubmission,
        #[serde(rename = "appClipDefaultExperience")]
        AppClipDefaultExperience,
        #[serde(rename = "appStoreVersionExperiments")]
        AppStoreVersionExperiments,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "alternativeDistributionPackage")]
        AlternativeDistributionPackage,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::VersionString => f.write_str("versionString"),
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::AppVersionState => f.write_str("appVersionState"),
                Self::Copyright => f.write_str("copyright"),
                Self::ReviewType => f.write_str("reviewType"),
                Self::ReleaseType => f.write_str("releaseType"),
                Self::EarliestReleaseDate => f.write_str("earliestReleaseDate"),
                Self::UsesIdfa => f.write_str("usesIdfa"),
                Self::Downloadable => f.write_str("downloadable"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::App => f.write_str("app"),
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
                Self::Build => f.write_str("build"),
                Self::AppStoreVersionPhasedRelease => f.write_str("appStoreVersionPhasedRelease"),
                Self::GameCenterAppVersion => f.write_str("gameCenterAppVersion"),
                Self::RoutingAppCoverage => f.write_str("routingAppCoverage"),
                Self::AppStoreReviewDetail => f.write_str("appStoreReviewDetail"),
                Self::AppStoreVersionSubmission => f.write_str("appStoreVersionSubmission"),
                Self::AppClipDefaultExperience => f.write_str("appClipDefaultExperience"),
                Self::AppStoreVersionExperiments => f.write_str("appStoreVersionExperiments"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::AlternativeDistributionPackage => {
                    f.write_str("alternativeDistributionPackage")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "versionString" => Ok(Self::VersionString),
                "appStoreState" => Ok(Self::AppStoreState),
                "appVersionState" => Ok(Self::AppVersionState),
                "copyright" => Ok(Self::Copyright),
                "reviewType" => Ok(Self::ReviewType),
                "releaseType" => Ok(Self::ReleaseType),
                "earliestReleaseDate" => Ok(Self::EarliestReleaseDate),
                "usesIdfa" => Ok(Self::UsesIdfa),
                "downloadable" => Ok(Self::Downloadable),
                "createdDate" => Ok(Self::CreatedDate),
                "app" => Ok(Self::App),
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                "build" => Ok(Self::Build),
                "appStoreVersionPhasedRelease" => Ok(Self::AppStoreVersionPhasedRelease),
                "gameCenterAppVersion" => Ok(Self::GameCenterAppVersion),
                "routingAppCoverage" => Ok(Self::RoutingAppCoverage),
                "appStoreReviewDetail" => Ok(Self::AppStoreReviewDetail),
                "appStoreVersionSubmission" => Ok(Self::AppStoreVersionSubmission),
                "appClipDefaultExperience" => Ok(Self::AppClipDefaultExperience),
                "appStoreVersionExperiments" => Ok(Self::AppStoreVersionExperiments),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "customerReviews" => Ok(Self::CustomerReviews),
                "alternativeDistributionPackage" => Ok(Self::AlternativeDistributionPackage),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "accessibilityUrl",
    ///    "name",
    ///    "bundleId",
    ///    "sku",
    ///    "primaryLocale",
    ///    "isOrEverWasMadeForKids",
    ///    "subscriptionStatusUrl",
    ///    "subscriptionStatusUrlVersion",
    ///    "subscriptionStatusUrlForSandbox",
    ///    "subscriptionStatusUrlVersionForSandbox",
    ///    "contentRightsDeclaration",
    ///    "streamlinedPurchasingEnabled",
    ///    "accessibilityDeclarations",
    ///    "appEncryptionDeclarations",
    ///    "appStoreIcon",
    ///    "ciProduct",
    ///    "betaTesters",
    ///    "betaGroups",
    ///    "appStoreVersions",
    ///    "appTags",
    ///    "preReleaseVersions",
    ///    "betaAppLocalizations",
    ///    "builds",
    ///    "betaLicenseAgreement",
    ///    "betaAppReviewDetail",
    ///    "appInfos",
    ///    "appClips",
    ///    "appPricePoints",
    ///    "endUserLicenseAgreement",
    ///    "appPriceSchedule",
    ///    "appAvailabilityV2",
    ///    "inAppPurchases",
    ///    "subscriptionGroups",
    ///    "gameCenterEnabledVersions",
    ///    "perfPowerMetrics",
    ///    "appCustomProductPages",
    ///    "inAppPurchasesV2",
    ///    "promotedPurchases",
    ///    "appEvents",
    ///    "reviewSubmissions",
    ///    "subscriptionGracePeriod",
    ///    "customerReviews",
    ///    "customerReviewSummarizations",
    ///    "gameCenterDetail",
    ///    "appStoreVersionExperimentsV2",
    ///    "alternativeDistributionKey",
    ///    "analyticsReportRequests",
    ///    "marketplaceSearchDetail",
    ///    "buildUploads",
    ///    "backgroundAssets",
    ///    "betaFeedbackScreenshotSubmissions",
    ///    "betaFeedbackCrashSubmissions",
    ///    "searchKeywords",
    ///    "webhooks",
    ///    "androidToIosAppMappingDetails"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem {
        #[serde(rename = "accessibilityUrl")]
        AccessibilityUrl,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "sku")]
        Sku,
        #[serde(rename = "primaryLocale")]
        PrimaryLocale,
        #[serde(rename = "isOrEverWasMadeForKids")]
        IsOrEverWasMadeForKids,
        #[serde(rename = "subscriptionStatusUrl")]
        SubscriptionStatusUrl,
        #[serde(rename = "subscriptionStatusUrlVersion")]
        SubscriptionStatusUrlVersion,
        #[serde(rename = "subscriptionStatusUrlForSandbox")]
        SubscriptionStatusUrlForSandbox,
        #[serde(rename = "subscriptionStatusUrlVersionForSandbox")]
        SubscriptionStatusUrlVersionForSandbox,
        #[serde(rename = "contentRightsDeclaration")]
        ContentRightsDeclaration,
        #[serde(rename = "streamlinedPurchasingEnabled")]
        StreamlinedPurchasingEnabled,
        #[serde(rename = "accessibilityDeclarations")]
        AccessibilityDeclarations,
        #[serde(rename = "appEncryptionDeclarations")]
        AppEncryptionDeclarations,
        #[serde(rename = "appStoreIcon")]
        AppStoreIcon,
        #[serde(rename = "ciProduct")]
        CiProduct,
        #[serde(rename = "betaTesters")]
        BetaTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
        #[serde(rename = "appTags")]
        AppTags,
        #[serde(rename = "preReleaseVersions")]
        PreReleaseVersions,
        #[serde(rename = "betaAppLocalizations")]
        BetaAppLocalizations,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaLicenseAgreement")]
        BetaLicenseAgreement,
        #[serde(rename = "betaAppReviewDetail")]
        BetaAppReviewDetail,
        #[serde(rename = "appInfos")]
        AppInfos,
        #[serde(rename = "appClips")]
        AppClips,
        #[serde(rename = "appPricePoints")]
        AppPricePoints,
        #[serde(rename = "endUserLicenseAgreement")]
        EndUserLicenseAgreement,
        #[serde(rename = "appPriceSchedule")]
        AppPriceSchedule,
        #[serde(rename = "appAvailabilityV2")]
        AppAvailabilityV2,
        #[serde(rename = "inAppPurchases")]
        InAppPurchases,
        #[serde(rename = "subscriptionGroups")]
        SubscriptionGroups,
        #[serde(rename = "gameCenterEnabledVersions")]
        GameCenterEnabledVersions,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "appCustomProductPages")]
        AppCustomProductPages,
        #[serde(rename = "inAppPurchasesV2")]
        InAppPurchasesV2,
        #[serde(rename = "promotedPurchases")]
        PromotedPurchases,
        #[serde(rename = "appEvents")]
        AppEvents,
        #[serde(rename = "reviewSubmissions")]
        ReviewSubmissions,
        #[serde(rename = "subscriptionGracePeriod")]
        SubscriptionGracePeriod,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "customerReviewSummarizations")]
        CustomerReviewSummarizations,
        #[serde(rename = "gameCenterDetail")]
        GameCenterDetail,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "alternativeDistributionKey")]
        AlternativeDistributionKey,
        #[serde(rename = "analyticsReportRequests")]
        AnalyticsReportRequests,
        #[serde(rename = "marketplaceSearchDetail")]
        MarketplaceSearchDetail,
        #[serde(rename = "buildUploads")]
        BuildUploads,
        #[serde(rename = "backgroundAssets")]
        BackgroundAssets,
        #[serde(rename = "betaFeedbackScreenshotSubmissions")]
        BetaFeedbackScreenshotSubmissions,
        #[serde(rename = "betaFeedbackCrashSubmissions")]
        BetaFeedbackCrashSubmissions,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
        #[serde(rename = "webhooks")]
        Webhooks,
        #[serde(rename = "androidToIosAppMappingDetails")]
        AndroidToIosAppMappingDetails,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AccessibilityUrl => f.write_str("accessibilityUrl"),
                Self::Name => f.write_str("name"),
                Self::BundleId => f.write_str("bundleId"),
                Self::Sku => f.write_str("sku"),
                Self::PrimaryLocale => f.write_str("primaryLocale"),
                Self::IsOrEverWasMadeForKids => f.write_str("isOrEverWasMadeForKids"),
                Self::SubscriptionStatusUrl => f.write_str("subscriptionStatusUrl"),
                Self::SubscriptionStatusUrlVersion => f.write_str("subscriptionStatusUrlVersion"),
                Self::SubscriptionStatusUrlForSandbox => {
                    f.write_str("subscriptionStatusUrlForSandbox")
                }
                Self::SubscriptionStatusUrlVersionForSandbox => {
                    f.write_str("subscriptionStatusUrlVersionForSandbox")
                }
                Self::ContentRightsDeclaration => f.write_str("contentRightsDeclaration"),
                Self::StreamlinedPurchasingEnabled => f.write_str("streamlinedPurchasingEnabled"),
                Self::AccessibilityDeclarations => f.write_str("accessibilityDeclarations"),
                Self::AppEncryptionDeclarations => f.write_str("appEncryptionDeclarations"),
                Self::AppStoreIcon => f.write_str("appStoreIcon"),
                Self::CiProduct => f.write_str("ciProduct"),
                Self::BetaTesters => f.write_str("betaTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
                Self::AppTags => f.write_str("appTags"),
                Self::PreReleaseVersions => f.write_str("preReleaseVersions"),
                Self::BetaAppLocalizations => f.write_str("betaAppLocalizations"),
                Self::Builds => f.write_str("builds"),
                Self::BetaLicenseAgreement => f.write_str("betaLicenseAgreement"),
                Self::BetaAppReviewDetail => f.write_str("betaAppReviewDetail"),
                Self::AppInfos => f.write_str("appInfos"),
                Self::AppClips => f.write_str("appClips"),
                Self::AppPricePoints => f.write_str("appPricePoints"),
                Self::EndUserLicenseAgreement => f.write_str("endUserLicenseAgreement"),
                Self::AppPriceSchedule => f.write_str("appPriceSchedule"),
                Self::AppAvailabilityV2 => f.write_str("appAvailabilityV2"),
                Self::InAppPurchases => f.write_str("inAppPurchases"),
                Self::SubscriptionGroups => f.write_str("subscriptionGroups"),
                Self::GameCenterEnabledVersions => f.write_str("gameCenterEnabledVersions"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::AppCustomProductPages => f.write_str("appCustomProductPages"),
                Self::InAppPurchasesV2 => f.write_str("inAppPurchasesV2"),
                Self::PromotedPurchases => f.write_str("promotedPurchases"),
                Self::AppEvents => f.write_str("appEvents"),
                Self::ReviewSubmissions => f.write_str("reviewSubmissions"),
                Self::SubscriptionGracePeriod => f.write_str("subscriptionGracePeriod"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::CustomerReviewSummarizations => f.write_str("customerReviewSummarizations"),
                Self::GameCenterDetail => f.write_str("gameCenterDetail"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AlternativeDistributionKey => f.write_str("alternativeDistributionKey"),
                Self::AnalyticsReportRequests => f.write_str("analyticsReportRequests"),
                Self::MarketplaceSearchDetail => f.write_str("marketplaceSearchDetail"),
                Self::BuildUploads => f.write_str("buildUploads"),
                Self::BackgroundAssets => f.write_str("backgroundAssets"),
                Self::BetaFeedbackScreenshotSubmissions => {
                    f.write_str("betaFeedbackScreenshotSubmissions")
                }
                Self::BetaFeedbackCrashSubmissions => f.write_str("betaFeedbackCrashSubmissions"),
                Self::SearchKeywords => f.write_str("searchKeywords"),
                Self::Webhooks => f.write_str("webhooks"),
                Self::AndroidToIosAppMappingDetails => f.write_str("androidToIosAppMappingDetails"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "accessibilityUrl" => Ok(Self::AccessibilityUrl),
                "name" => Ok(Self::Name),
                "bundleId" => Ok(Self::BundleId),
                "sku" => Ok(Self::Sku),
                "primaryLocale" => Ok(Self::PrimaryLocale),
                "isOrEverWasMadeForKids" => Ok(Self::IsOrEverWasMadeForKids),
                "subscriptionStatusUrl" => Ok(Self::SubscriptionStatusUrl),
                "subscriptionStatusUrlVersion" => Ok(Self::SubscriptionStatusUrlVersion),
                "subscriptionStatusUrlForSandbox" => Ok(Self::SubscriptionStatusUrlForSandbox),
                "subscriptionStatusUrlVersionForSandbox" => {
                    Ok(Self::SubscriptionStatusUrlVersionForSandbox)
                }
                "contentRightsDeclaration" => Ok(Self::ContentRightsDeclaration),
                "streamlinedPurchasingEnabled" => Ok(Self::StreamlinedPurchasingEnabled),
                "accessibilityDeclarations" => Ok(Self::AccessibilityDeclarations),
                "appEncryptionDeclarations" => Ok(Self::AppEncryptionDeclarations),
                "appStoreIcon" => Ok(Self::AppStoreIcon),
                "ciProduct" => Ok(Self::CiProduct),
                "betaTesters" => Ok(Self::BetaTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                "appTags" => Ok(Self::AppTags),
                "preReleaseVersions" => Ok(Self::PreReleaseVersions),
                "betaAppLocalizations" => Ok(Self::BetaAppLocalizations),
                "builds" => Ok(Self::Builds),
                "betaLicenseAgreement" => Ok(Self::BetaLicenseAgreement),
                "betaAppReviewDetail" => Ok(Self::BetaAppReviewDetail),
                "appInfos" => Ok(Self::AppInfos),
                "appClips" => Ok(Self::AppClips),
                "appPricePoints" => Ok(Self::AppPricePoints),
                "endUserLicenseAgreement" => Ok(Self::EndUserLicenseAgreement),
                "appPriceSchedule" => Ok(Self::AppPriceSchedule),
                "appAvailabilityV2" => Ok(Self::AppAvailabilityV2),
                "inAppPurchases" => Ok(Self::InAppPurchases),
                "subscriptionGroups" => Ok(Self::SubscriptionGroups),
                "gameCenterEnabledVersions" => Ok(Self::GameCenterEnabledVersions),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "appCustomProductPages" => Ok(Self::AppCustomProductPages),
                "inAppPurchasesV2" => Ok(Self::InAppPurchasesV2),
                "promotedPurchases" => Ok(Self::PromotedPurchases),
                "appEvents" => Ok(Self::AppEvents),
                "reviewSubmissions" => Ok(Self::ReviewSubmissions),
                "subscriptionGracePeriod" => Ok(Self::SubscriptionGracePeriod),
                "customerReviews" => Ok(Self::CustomerReviews),
                "customerReviewSummarizations" => Ok(Self::CustomerReviewSummarizations),
                "gameCenterDetail" => Ok(Self::GameCenterDetail),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "alternativeDistributionKey" => Ok(Self::AlternativeDistributionKey),
                "analyticsReportRequests" => Ok(Self::AnalyticsReportRequests),
                "marketplaceSearchDetail" => Ok(Self::MarketplaceSearchDetail),
                "buildUploads" => Ok(Self::BuildUploads),
                "backgroundAssets" => Ok(Self::BackgroundAssets),
                "betaFeedbackScreenshotSubmissions" => Ok(Self::BetaFeedbackScreenshotSubmissions),
                "betaFeedbackCrashSubmissions" => Ok(Self::BetaFeedbackCrashSubmissions),
                "searchKeywords" => Ok(Self::SearchKeywords),
                "webhooks" => Ok(Self::Webhooks),
                "androidToIosAppMappingDetails" => Ok(Self::AndroidToIosAppMappingDetails),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "version",
    ///    "uploadedDate",
    ///    "expirationDate",
    ///    "expired",
    ///    "minOsVersion",
    ///    "lsMinimumSystemVersion",
    ///    "computedMinMacOsVersion",
    ///    "computedMinVisionOsVersion",
    ///    "iconAssetToken",
    ///    "processingState",
    ///    "buildAudienceType",
    ///    "usesNonExemptEncryption",
    ///    "preReleaseVersion",
    ///    "individualTesters",
    ///    "betaGroups",
    ///    "betaBuildLocalizations",
    ///    "appEncryptionDeclaration",
    ///    "betaAppReviewSubmission",
    ///    "app",
    ///    "buildBetaDetail",
    ///    "appStoreVersion",
    ///    "icons",
    ///    "buildBundles",
    ///    "buildUpload",
    ///    "perfPowerMetrics",
    ///    "diagnosticSignatures"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem {
        #[serde(rename = "version")]
        Version,
        #[serde(rename = "uploadedDate")]
        UploadedDate,
        #[serde(rename = "expirationDate")]
        ExpirationDate,
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "minOsVersion")]
        MinOsVersion,
        #[serde(rename = "lsMinimumSystemVersion")]
        LsMinimumSystemVersion,
        #[serde(rename = "computedMinMacOsVersion")]
        ComputedMinMacOsVersion,
        #[serde(rename = "computedMinVisionOsVersion")]
        ComputedMinVisionOsVersion,
        #[serde(rename = "iconAssetToken")]
        IconAssetToken,
        #[serde(rename = "processingState")]
        ProcessingState,
        #[serde(rename = "buildAudienceType")]
        BuildAudienceType,
        #[serde(rename = "usesNonExemptEncryption")]
        UsesNonExemptEncryption,
        #[serde(rename = "preReleaseVersion")]
        PreReleaseVersion,
        #[serde(rename = "individualTesters")]
        IndividualTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "betaBuildLocalizations")]
        BetaBuildLocalizations,
        #[serde(rename = "appEncryptionDeclaration")]
        AppEncryptionDeclaration,
        #[serde(rename = "betaAppReviewSubmission")]
        BetaAppReviewSubmission,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "buildBetaDetail")]
        BuildBetaDetail,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "icons")]
        Icons,
        #[serde(rename = "buildBundles")]
        BuildBundles,
        #[serde(rename = "buildUpload")]
        BuildUpload,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "diagnosticSignatures")]
        DiagnosticSignatures,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Version => f.write_str("version"),
                Self::UploadedDate => f.write_str("uploadedDate"),
                Self::ExpirationDate => f.write_str("expirationDate"),
                Self::Expired => f.write_str("expired"),
                Self::MinOsVersion => f.write_str("minOsVersion"),
                Self::LsMinimumSystemVersion => f.write_str("lsMinimumSystemVersion"),
                Self::ComputedMinMacOsVersion => f.write_str("computedMinMacOsVersion"),
                Self::ComputedMinVisionOsVersion => f.write_str("computedMinVisionOsVersion"),
                Self::IconAssetToken => f.write_str("iconAssetToken"),
                Self::ProcessingState => f.write_str("processingState"),
                Self::BuildAudienceType => f.write_str("buildAudienceType"),
                Self::UsesNonExemptEncryption => f.write_str("usesNonExemptEncryption"),
                Self::PreReleaseVersion => f.write_str("preReleaseVersion"),
                Self::IndividualTesters => f.write_str("individualTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::BetaBuildLocalizations => f.write_str("betaBuildLocalizations"),
                Self::AppEncryptionDeclaration => f.write_str("appEncryptionDeclaration"),
                Self::BetaAppReviewSubmission => f.write_str("betaAppReviewSubmission"),
                Self::App => f.write_str("app"),
                Self::BuildBetaDetail => f.write_str("buildBetaDetail"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::Icons => f.write_str("icons"),
                Self::BuildBundles => f.write_str("buildBundles"),
                Self::BuildUpload => f.write_str("buildUpload"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::DiagnosticSignatures => f.write_str("diagnosticSignatures"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "version" => Ok(Self::Version),
                "uploadedDate" => Ok(Self::UploadedDate),
                "expirationDate" => Ok(Self::ExpirationDate),
                "expired" => Ok(Self::Expired),
                "minOsVersion" => Ok(Self::MinOsVersion),
                "lsMinimumSystemVersion" => Ok(Self::LsMinimumSystemVersion),
                "computedMinMacOsVersion" => Ok(Self::ComputedMinMacOsVersion),
                "computedMinVisionOsVersion" => Ok(Self::ComputedMinVisionOsVersion),
                "iconAssetToken" => Ok(Self::IconAssetToken),
                "processingState" => Ok(Self::ProcessingState),
                "buildAudienceType" => Ok(Self::BuildAudienceType),
                "usesNonExemptEncryption" => Ok(Self::UsesNonExemptEncryption),
                "preReleaseVersion" => Ok(Self::PreReleaseVersion),
                "individualTesters" => Ok(Self::IndividualTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "betaBuildLocalizations" => Ok(Self::BetaBuildLocalizations),
                "appEncryptionDeclaration" => Ok(Self::AppEncryptionDeclaration),
                "betaAppReviewSubmission" => Ok(Self::BetaAppReviewSubmission),
                "app" => Ok(Self::App),
                "buildBetaDetail" => Ok(Self::BuildBetaDetail),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "icons" => Ok(Self::Icons),
                "buildBundles" => Ok(Self::BuildBundles),
                "buildUpload" => Ok(Self::BuildUpload),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "diagnosticSignatures" => Ok(Self::DiagnosticSignatures),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "enabled",
    ///    "compatibilityVersions",
    ///    "appStoreVersion"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "compatibilityVersions")]
        CompatibilityVersions,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Enabled => f.write_str("enabled"),
                Self::CompatibilityVersions => f.write_str("compatibilityVersions"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "enabled" => Ok(Self::Enabled),
                "compatibilityVersions" => Ok(Self::CompatibilityVersions),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "fileSize",
    ///    "fileName",
    ///    "sourceFileChecksum",
    ///    "uploadOperations",
    ///    "assetDeliveryState",
    ///    "appStoreVersion"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem {
        #[serde(rename = "fileSize")]
        FileSize,
        #[serde(rename = "fileName")]
        FileName,
        #[serde(rename = "sourceFileChecksum")]
        SourceFileChecksum,
        #[serde(rename = "uploadOperations")]
        UploadOperations,
        #[serde(rename = "assetDeliveryState")]
        AssetDeliveryState,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::FileSize => f.write_str("fileSize"),
                Self::FileName => f.write_str("fileName"),
                Self::SourceFileChecksum => f.write_str("sourceFileChecksum"),
                Self::UploadOperations => f.write_str("uploadOperations"),
                Self::AssetDeliveryState => f.write_str("assetDeliveryState"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "fileSize" => Ok(Self::FileSize),
                "fileName" => Ok(Self::FileName),
                "sourceFileChecksum" => Ok(Self::SourceFileChecksum),
                "uploadOperations" => Ok(Self::UploadOperations),
                "assetDeliveryState" => Ok(Self::AssetDeliveryState),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACCEPTED",
    ///    "DEVELOPER_REMOVED_FROM_SALE",
    ///    "DEVELOPER_REJECTED",
    ///    "IN_REVIEW",
    ///    "INVALID_BINARY",
    ///    "METADATA_REJECTED",
    ///    "PENDING_APPLE_RELEASE",
    ///    "PENDING_CONTRACT",
    ///    "PENDING_DEVELOPER_RELEASE",
    ///    "PREPARE_FOR_SUBMISSION",
    ///    "PREORDER_READY_FOR_SALE",
    ///    "PROCESSING_FOR_APP_STORE",
    ///    "READY_FOR_REVIEW",
    ///    "READY_FOR_SALE",
    ///    "REJECTED",
    ///    "REMOVED_FROM_SALE",
    ///    "WAITING_FOR_EXPORT_COMPLIANCE",
    ///    "WAITING_FOR_REVIEW",
    ///    "REPLACED_WITH_NEW_VERSION",
    ///    "NOT_APPLICABLE"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem {
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "DEVELOPER_REMOVED_FROM_SALE")]
        DeveloperRemovedFromSale,
        #[serde(rename = "DEVELOPER_REJECTED")]
        DeveloperRejected,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "INVALID_BINARY")]
        InvalidBinary,
        #[serde(rename = "METADATA_REJECTED")]
        MetadataRejected,
        #[serde(rename = "PENDING_APPLE_RELEASE")]
        PendingAppleRelease,
        #[serde(rename = "PENDING_CONTRACT")]
        PendingContract,
        #[serde(rename = "PENDING_DEVELOPER_RELEASE")]
        PendingDeveloperRelease,
        #[serde(rename = "PREPARE_FOR_SUBMISSION")]
        PrepareForSubmission,
        #[serde(rename = "PREORDER_READY_FOR_SALE")]
        PreorderReadyForSale,
        #[serde(rename = "PROCESSING_FOR_APP_STORE")]
        ProcessingForAppStore,
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "READY_FOR_SALE")]
        ReadyForSale,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "REMOVED_FROM_SALE")]
        RemovedFromSale,
        #[serde(rename = "WAITING_FOR_EXPORT_COMPLIANCE")]
        WaitingForExportCompliance,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
        #[serde(rename = "REPLACED_WITH_NEW_VERSION")]
        ReplacedWithNewVersion,
        #[serde(rename = "NOT_APPLICABLE")]
        NotApplicable,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::DeveloperRemovedFromSale => f.write_str("DEVELOPER_REMOVED_FROM_SALE"),
                Self::DeveloperRejected => f.write_str("DEVELOPER_REJECTED"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::InvalidBinary => f.write_str("INVALID_BINARY"),
                Self::MetadataRejected => f.write_str("METADATA_REJECTED"),
                Self::PendingAppleRelease => f.write_str("PENDING_APPLE_RELEASE"),
                Self::PendingContract => f.write_str("PENDING_CONTRACT"),
                Self::PendingDeveloperRelease => f.write_str("PENDING_DEVELOPER_RELEASE"),
                Self::PrepareForSubmission => f.write_str("PREPARE_FOR_SUBMISSION"),
                Self::PreorderReadyForSale => f.write_str("PREORDER_READY_FOR_SALE"),
                Self::ProcessingForAppStore => f.write_str("PROCESSING_FOR_APP_STORE"),
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::ReadyForSale => f.write_str("READY_FOR_SALE"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::RemovedFromSale => f.write_str("REMOVED_FROM_SALE"),
                Self::WaitingForExportCompliance => f.write_str("WAITING_FOR_EXPORT_COMPLIANCE"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
                Self::ReplacedWithNewVersion => f.write_str("REPLACED_WITH_NEW_VERSION"),
                Self::NotApplicable => f.write_str("NOT_APPLICABLE"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACCEPTED" => Ok(Self::Accepted),
                "DEVELOPER_REMOVED_FROM_SALE" => Ok(Self::DeveloperRemovedFromSale),
                "DEVELOPER_REJECTED" => Ok(Self::DeveloperRejected),
                "IN_REVIEW" => Ok(Self::InReview),
                "INVALID_BINARY" => Ok(Self::InvalidBinary),
                "METADATA_REJECTED" => Ok(Self::MetadataRejected),
                "PENDING_APPLE_RELEASE" => Ok(Self::PendingAppleRelease),
                "PENDING_CONTRACT" => Ok(Self::PendingContract),
                "PENDING_DEVELOPER_RELEASE" => Ok(Self::PendingDeveloperRelease),
                "PREPARE_FOR_SUBMISSION" => Ok(Self::PrepareForSubmission),
                "PREORDER_READY_FOR_SALE" => Ok(Self::PreorderReadyForSale),
                "PROCESSING_FOR_APP_STORE" => Ok(Self::ProcessingForAppStore),
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "READY_FOR_SALE" => Ok(Self::ReadyForSale),
                "REJECTED" => Ok(Self::Rejected),
                "REMOVED_FROM_SALE" => Ok(Self::RemovedFromSale),
                "WAITING_FOR_EXPORT_COMPLIANCE" => Ok(Self::WaitingForExportCompliance),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                "REPLACED_WITH_NEW_VERSION" => Ok(Self::ReplacedWithNewVersion),
                "NOT_APPLICABLE" => Ok(Self::NotApplicable),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACCEPTED",
    ///    "DEVELOPER_REJECTED",
    ///    "IN_REVIEW",
    ///    "INVALID_BINARY",
    ///    "METADATA_REJECTED",
    ///    "PENDING_APPLE_RELEASE",
    ///    "PENDING_DEVELOPER_RELEASE",
    ///    "PREPARE_FOR_SUBMISSION",
    ///    "PROCESSING_FOR_DISTRIBUTION",
    ///    "READY_FOR_DISTRIBUTION",
    ///    "READY_FOR_REVIEW",
    ///    "REJECTED",
    ///    "REPLACED_WITH_NEW_VERSION",
    ///    "WAITING_FOR_EXPORT_COMPLIANCE",
    ///    "WAITING_FOR_REVIEW"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem {
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "DEVELOPER_REJECTED")]
        DeveloperRejected,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "INVALID_BINARY")]
        InvalidBinary,
        #[serde(rename = "METADATA_REJECTED")]
        MetadataRejected,
        #[serde(rename = "PENDING_APPLE_RELEASE")]
        PendingAppleRelease,
        #[serde(rename = "PENDING_DEVELOPER_RELEASE")]
        PendingDeveloperRelease,
        #[serde(rename = "PREPARE_FOR_SUBMISSION")]
        PrepareForSubmission,
        #[serde(rename = "PROCESSING_FOR_DISTRIBUTION")]
        ProcessingForDistribution,
        #[serde(rename = "READY_FOR_DISTRIBUTION")]
        ReadyForDistribution,
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "REPLACED_WITH_NEW_VERSION")]
        ReplacedWithNewVersion,
        #[serde(rename = "WAITING_FOR_EXPORT_COMPLIANCE")]
        WaitingForExportCompliance,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::DeveloperRejected => f.write_str("DEVELOPER_REJECTED"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::InvalidBinary => f.write_str("INVALID_BINARY"),
                Self::MetadataRejected => f.write_str("METADATA_REJECTED"),
                Self::PendingAppleRelease => f.write_str("PENDING_APPLE_RELEASE"),
                Self::PendingDeveloperRelease => f.write_str("PENDING_DEVELOPER_RELEASE"),
                Self::PrepareForSubmission => f.write_str("PREPARE_FOR_SUBMISSION"),
                Self::ProcessingForDistribution => f.write_str("PROCESSING_FOR_DISTRIBUTION"),
                Self::ReadyForDistribution => f.write_str("READY_FOR_DISTRIBUTION"),
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::ReplacedWithNewVersion => f.write_str("REPLACED_WITH_NEW_VERSION"),
                Self::WaitingForExportCompliance => f.write_str("WAITING_FOR_EXPORT_COMPLIANCE"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACCEPTED" => Ok(Self::Accepted),
                "DEVELOPER_REJECTED" => Ok(Self::DeveloperRejected),
                "IN_REVIEW" => Ok(Self::InReview),
                "INVALID_BINARY" => Ok(Self::InvalidBinary),
                "METADATA_REJECTED" => Ok(Self::MetadataRejected),
                "PENDING_APPLE_RELEASE" => Ok(Self::PendingAppleRelease),
                "PENDING_DEVELOPER_RELEASE" => Ok(Self::PendingDeveloperRelease),
                "PREPARE_FOR_SUBMISSION" => Ok(Self::PrepareForSubmission),
                "PROCESSING_FOR_DISTRIBUTION" => Ok(Self::ProcessingForDistribution),
                "READY_FOR_DISTRIBUTION" => Ok(Self::ReadyForDistribution),
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "REJECTED" => Ok(Self::Rejected),
                "REPLACED_WITH_NEW_VERSION" => Ok(Self::ReplacedWithNewVersion),
                "WAITING_FOR_EXPORT_COMPLIANCE" => Ok(Self::WaitingForExportCompliance),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "IOS",
    ///    "MAC_OS",
    ///    "TV_OS",
    ///    "VISION_OS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem {
        #[serde(rename = "IOS")]
        Ios,
        #[serde(rename = "MAC_OS")]
        MacOs,
        #[serde(rename = "TV_OS")]
        TvOs,
        #[serde(rename = "VISION_OS")]
        VisionOs,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ios => f.write_str("IOS"),
                Self::MacOs => f.write_str("MAC_OS"),
                Self::TvOs => f.write_str("TV_OS"),
                Self::VisionOs => f.write_str("VISION_OS"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "IOS" => Ok(Self::Ios),
                "MAC_OS" => Ok(Self::MacOs),
                "TV_OS" => Ok(Self::TvOs),
                "VISION_OS" => Ok(Self::VisionOs),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedIncludeItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "app",
    ///    "appStoreVersionLocalizations",
    ///    "build",
    ///    "appStoreVersionPhasedRelease",
    ///    "gameCenterAppVersion",
    ///    "routingAppCoverage",
    ///    "appStoreReviewDetail",
    ///    "appStoreVersionSubmission",
    ///    "appClipDefaultExperience",
    ///    "appStoreVersionExperiments",
    ///    "appStoreVersionExperimentsV2",
    ///    "alternativeDistributionPackage"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedIncludeItem {
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "appStoreVersionPhasedRelease")]
        AppStoreVersionPhasedRelease,
        #[serde(rename = "gameCenterAppVersion")]
        GameCenterAppVersion,
        #[serde(rename = "routingAppCoverage")]
        RoutingAppCoverage,
        #[serde(rename = "appStoreReviewDetail")]
        AppStoreReviewDetail,
        #[serde(rename = "appStoreVersionSubmission")]
        AppStoreVersionSubmission,
        #[serde(rename = "appClipDefaultExperience")]
        AppClipDefaultExperience,
        #[serde(rename = "appStoreVersionExperiments")]
        AppStoreVersionExperiments,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "alternativeDistributionPackage")]
        AlternativeDistributionPackage,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedIncludeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::App => f.write_str("app"),
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
                Self::Build => f.write_str("build"),
                Self::AppStoreVersionPhasedRelease => f.write_str("appStoreVersionPhasedRelease"),
                Self::GameCenterAppVersion => f.write_str("gameCenterAppVersion"),
                Self::RoutingAppCoverage => f.write_str("routingAppCoverage"),
                Self::AppStoreReviewDetail => f.write_str("appStoreReviewDetail"),
                Self::AppStoreVersionSubmission => f.write_str("appStoreVersionSubmission"),
                Self::AppClipDefaultExperience => f.write_str("appClipDefaultExperience"),
                Self::AppStoreVersionExperiments => f.write_str("appStoreVersionExperiments"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AlternativeDistributionPackage => {
                    f.write_str("alternativeDistributionPackage")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedIncludeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "app" => Ok(Self::App),
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                "build" => Ok(Self::Build),
                "appStoreVersionPhasedRelease" => Ok(Self::AppStoreVersionPhasedRelease),
                "gameCenterAppVersion" => Ok(Self::GameCenterAppVersion),
                "routingAppCoverage" => Ok(Self::RoutingAppCoverage),
                "appStoreReviewDetail" => Ok(Self::AppStoreReviewDetail),
                "appStoreVersionSubmission" => Ok(Self::AppStoreVersionSubmission),
                "appClipDefaultExperience" => Ok(Self::AppClipDefaultExperience),
                "appStoreVersionExperiments" => Ok(Self::AppStoreVersionExperiments),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "alternativeDistributionPackage" => Ok(Self::AlternativeDistributionPackage),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppStoreVersionsGetToManyRelatedIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedIncludeItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedIncludeItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "packageName",
    ///    "appSigningKeyPublicCertificateSha256Fingerprints"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem {
        #[serde(rename = "packageName")]
        PackageName,
        #[serde(rename = "appSigningKeyPublicCertificateSha256Fingerprints")]
        AppSigningKeyPublicCertificateSha256Fingerprints,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::PackageName => f.write_str("packageName"),
                Self::AppSigningKeyPublicCertificateSha256Fingerprints => {
                    f.write_str("appSigningKeyPublicCertificateSha256Fingerprints")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "packageName" => Ok(Self::PackageName),
                "appSigningKeyPublicCertificateSha256Fingerprints" => {
                    Ok(Self::AppSigningKeyPublicCertificateSha256Fingerprints)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppClipsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "bundleId",
    ///    "app",
    ///    "appClipDefaultExperiences",
    ///    "appClipAdvancedExperiences"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppClipsItem {
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appClipDefaultExperiences")]
        AppClipDefaultExperiences,
        #[serde(rename = "appClipAdvancedExperiences")]
        AppClipAdvancedExperiences,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppClipsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::BundleId => f.write_str("bundleId"),
                Self::App => f.write_str("app"),
                Self::AppClipDefaultExperiences => f.write_str("appClipDefaultExperiences"),
                Self::AppClipAdvancedExperiences => f.write_str("appClipAdvancedExperiences"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppClipsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "bundleId" => Ok(Self::BundleId),
                "app" => Ok(Self::App),
                "appClipDefaultExperiences" => Ok(Self::AppClipDefaultExperiences),
                "appClipAdvancedExperiences" => Ok(Self::AppClipAdvancedExperiences),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppClipsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsAppClipsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsAppClipsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppCustomProductPagesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "url",
    ///    "visible",
    ///    "app",
    ///    "appCustomProductPageVersions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppCustomProductPagesItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "url")]
        Url,
        #[serde(rename = "visible")]
        Visible,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appCustomProductPageVersions")]
        AppCustomProductPageVersions,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppCustomProductPagesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::Url => f.write_str("url"),
                Self::Visible => f.write_str("visible"),
                Self::App => f.write_str("app"),
                Self::AppCustomProductPageVersions => f.write_str("appCustomProductPageVersions"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppCustomProductPagesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "url" => Ok(Self::Url),
                "visible" => Ok(Self::Visible),
                "app" => Ok(Self::App),
                "appCustomProductPageVersions" => Ok(Self::AppCustomProductPageVersions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppCustomProductPagesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsAppCustomProductPagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsAppCustomProductPagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppEncryptionDeclarationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appDescription",
    ///    "createdDate",
    ///    "usesEncryption",
    ///    "exempt",
    ///    "containsProprietaryCryptography",
    ///    "containsThirdPartyCryptography",
    ///    "availableOnFrenchStore",
    ///    "platform",
    ///    "uploadedDate",
    ///    "documentUrl",
    ///    "documentName",
    ///    "documentType",
    ///    "appEncryptionDeclarationState",
    ///    "codeValue",
    ///    "app",
    ///    "builds",
    ///    "appEncryptionDeclarationDocument"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppEncryptionDeclarationsItem {
        #[serde(rename = "appDescription")]
        AppDescription,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "usesEncryption")]
        UsesEncryption,
        #[serde(rename = "exempt")]
        Exempt,
        #[serde(rename = "containsProprietaryCryptography")]
        ContainsProprietaryCryptography,
        #[serde(rename = "containsThirdPartyCryptography")]
        ContainsThirdPartyCryptography,
        #[serde(rename = "availableOnFrenchStore")]
        AvailableOnFrenchStore,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "uploadedDate")]
        UploadedDate,
        #[serde(rename = "documentUrl")]
        DocumentUrl,
        #[serde(rename = "documentName")]
        DocumentName,
        #[serde(rename = "documentType")]
        DocumentType,
        #[serde(rename = "appEncryptionDeclarationState")]
        AppEncryptionDeclarationState,
        #[serde(rename = "codeValue")]
        CodeValue,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "appEncryptionDeclarationDocument")]
        AppEncryptionDeclarationDocument,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppEncryptionDeclarationsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppDescription => f.write_str("appDescription"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::UsesEncryption => f.write_str("usesEncryption"),
                Self::Exempt => f.write_str("exempt"),
                Self::ContainsProprietaryCryptography => {
                    f.write_str("containsProprietaryCryptography")
                }
                Self::ContainsThirdPartyCryptography => {
                    f.write_str("containsThirdPartyCryptography")
                }
                Self::AvailableOnFrenchStore => f.write_str("availableOnFrenchStore"),
                Self::Platform => f.write_str("platform"),
                Self::UploadedDate => f.write_str("uploadedDate"),
                Self::DocumentUrl => f.write_str("documentUrl"),
                Self::DocumentName => f.write_str("documentName"),
                Self::DocumentType => f.write_str("documentType"),
                Self::AppEncryptionDeclarationState => f.write_str("appEncryptionDeclarationState"),
                Self::CodeValue => f.write_str("codeValue"),
                Self::App => f.write_str("app"),
                Self::Builds => f.write_str("builds"),
                Self::AppEncryptionDeclarationDocument => {
                    f.write_str("appEncryptionDeclarationDocument")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppEncryptionDeclarationsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appDescription" => Ok(Self::AppDescription),
                "createdDate" => Ok(Self::CreatedDate),
                "usesEncryption" => Ok(Self::UsesEncryption),
                "exempt" => Ok(Self::Exempt),
                "containsProprietaryCryptography" => Ok(Self::ContainsProprietaryCryptography),
                "containsThirdPartyCryptography" => Ok(Self::ContainsThirdPartyCryptography),
                "availableOnFrenchStore" => Ok(Self::AvailableOnFrenchStore),
                "platform" => Ok(Self::Platform),
                "uploadedDate" => Ok(Self::UploadedDate),
                "documentUrl" => Ok(Self::DocumentUrl),
                "documentName" => Ok(Self::DocumentName),
                "documentType" => Ok(Self::DocumentType),
                "appEncryptionDeclarationState" => Ok(Self::AppEncryptionDeclarationState),
                "codeValue" => Ok(Self::CodeValue),
                "app" => Ok(Self::App),
                "builds" => Ok(Self::Builds),
                "appEncryptionDeclarationDocument" => Ok(Self::AppEncryptionDeclarationDocument),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppEncryptionDeclarationsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsAppEncryptionDeclarationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsAppEncryptionDeclarationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppEventsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "referenceName",
    ///    "badge",
    ///    "eventState",
    ///    "deepLink",
    ///    "purchaseRequirement",
    ///    "primaryLocale",
    ///    "priority",
    ///    "purpose",
    ///    "territorySchedules",
    ///    "archivedTerritorySchedules",
    ///    "localizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppEventsItem {
        #[serde(rename = "referenceName")]
        ReferenceName,
        #[serde(rename = "badge")]
        Badge,
        #[serde(rename = "eventState")]
        EventState,
        #[serde(rename = "deepLink")]
        DeepLink,
        #[serde(rename = "purchaseRequirement")]
        PurchaseRequirement,
        #[serde(rename = "primaryLocale")]
        PrimaryLocale,
        #[serde(rename = "priority")]
        Priority,
        #[serde(rename = "purpose")]
        Purpose,
        #[serde(rename = "territorySchedules")]
        TerritorySchedules,
        #[serde(rename = "archivedTerritorySchedules")]
        ArchivedTerritorySchedules,
        #[serde(rename = "localizations")]
        Localizations,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppEventsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReferenceName => f.write_str("referenceName"),
                Self::Badge => f.write_str("badge"),
                Self::EventState => f.write_str("eventState"),
                Self::DeepLink => f.write_str("deepLink"),
                Self::PurchaseRequirement => f.write_str("purchaseRequirement"),
                Self::PrimaryLocale => f.write_str("primaryLocale"),
                Self::Priority => f.write_str("priority"),
                Self::Purpose => f.write_str("purpose"),
                Self::TerritorySchedules => f.write_str("territorySchedules"),
                Self::ArchivedTerritorySchedules => f.write_str("archivedTerritorySchedules"),
                Self::Localizations => f.write_str("localizations"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppEventsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "referenceName" => Ok(Self::ReferenceName),
                "badge" => Ok(Self::Badge),
                "eventState" => Ok(Self::EventState),
                "deepLink" => Ok(Self::DeepLink),
                "purchaseRequirement" => Ok(Self::PurchaseRequirement),
                "primaryLocale" => Ok(Self::PrimaryLocale),
                "priority" => Ok(Self::Priority),
                "purpose" => Ok(Self::Purpose),
                "territorySchedules" => Ok(Self::TerritorySchedules),
                "archivedTerritorySchedules" => Ok(Self::ArchivedTerritorySchedules),
                "localizations" => Ok(Self::Localizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsAppEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsAppEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppInfosItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreState",
    ///    "state",
    ///    "appStoreAgeRating",
    ///    "australiaAgeRating",
    ///    "brazilAgeRating",
    ///    "brazilAgeRatingV2",
    ///    "franceAgeRating",
    ///    "koreaAgeRating",
    ///    "app",
    ///    "ageRatingDeclaration",
    ///    "appInfoLocalizations",
    ///    "primaryCategory",
    ///    "primarySubcategoryOne",
    ///    "primarySubcategoryTwo",
    ///    "secondaryCategory",
    ///    "secondarySubcategoryOne",
    ///    "secondarySubcategoryTwo",
    ///    "territoryAgeRatings"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppInfosItem {
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "appStoreAgeRating")]
        AppStoreAgeRating,
        #[serde(rename = "australiaAgeRating")]
        AustraliaAgeRating,
        #[serde(rename = "brazilAgeRating")]
        BrazilAgeRating,
        #[serde(rename = "brazilAgeRatingV2")]
        BrazilAgeRatingV2,
        #[serde(rename = "franceAgeRating")]
        FranceAgeRating,
        #[serde(rename = "koreaAgeRating")]
        KoreaAgeRating,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "ageRatingDeclaration")]
        AgeRatingDeclaration,
        #[serde(rename = "appInfoLocalizations")]
        AppInfoLocalizations,
        #[serde(rename = "primaryCategory")]
        PrimaryCategory,
        #[serde(rename = "primarySubcategoryOne")]
        PrimarySubcategoryOne,
        #[serde(rename = "primarySubcategoryTwo")]
        PrimarySubcategoryTwo,
        #[serde(rename = "secondaryCategory")]
        SecondaryCategory,
        #[serde(rename = "secondarySubcategoryOne")]
        SecondarySubcategoryOne,
        #[serde(rename = "secondarySubcategoryTwo")]
        SecondarySubcategoryTwo,
        #[serde(rename = "territoryAgeRatings")]
        TerritoryAgeRatings,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppInfosItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::State => f.write_str("state"),
                Self::AppStoreAgeRating => f.write_str("appStoreAgeRating"),
                Self::AustraliaAgeRating => f.write_str("australiaAgeRating"),
                Self::BrazilAgeRating => f.write_str("brazilAgeRating"),
                Self::BrazilAgeRatingV2 => f.write_str("brazilAgeRatingV2"),
                Self::FranceAgeRating => f.write_str("franceAgeRating"),
                Self::KoreaAgeRating => f.write_str("koreaAgeRating"),
                Self::App => f.write_str("app"),
                Self::AgeRatingDeclaration => f.write_str("ageRatingDeclaration"),
                Self::AppInfoLocalizations => f.write_str("appInfoLocalizations"),
                Self::PrimaryCategory => f.write_str("primaryCategory"),
                Self::PrimarySubcategoryOne => f.write_str("primarySubcategoryOne"),
                Self::PrimarySubcategoryTwo => f.write_str("primarySubcategoryTwo"),
                Self::SecondaryCategory => f.write_str("secondaryCategory"),
                Self::SecondarySubcategoryOne => f.write_str("secondarySubcategoryOne"),
                Self::SecondarySubcategoryTwo => f.write_str("secondarySubcategoryTwo"),
                Self::TerritoryAgeRatings => f.write_str("territoryAgeRatings"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppInfosItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreState" => Ok(Self::AppStoreState),
                "state" => Ok(Self::State),
                "appStoreAgeRating" => Ok(Self::AppStoreAgeRating),
                "australiaAgeRating" => Ok(Self::AustraliaAgeRating),
                "brazilAgeRating" => Ok(Self::BrazilAgeRating),
                "brazilAgeRatingV2" => Ok(Self::BrazilAgeRatingV2),
                "franceAgeRating" => Ok(Self::FranceAgeRating),
                "koreaAgeRating" => Ok(Self::KoreaAgeRating),
                "app" => Ok(Self::App),
                "ageRatingDeclaration" => Ok(Self::AgeRatingDeclaration),
                "appInfoLocalizations" => Ok(Self::AppInfoLocalizations),
                "primaryCategory" => Ok(Self::PrimaryCategory),
                "primarySubcategoryOne" => Ok(Self::PrimarySubcategoryOne),
                "primarySubcategoryTwo" => Ok(Self::PrimarySubcategoryTwo),
                "secondaryCategory" => Ok(Self::SecondaryCategory),
                "secondarySubcategoryOne" => Ok(Self::SecondarySubcategoryOne),
                "secondarySubcategoryTwo" => Ok(Self::SecondarySubcategoryTwo),
                "territoryAgeRatings" => Ok(Self::TerritoryAgeRatings),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppStoreVersionExperimentsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "platform",
    ///    "trafficProportion",
    ///    "state",
    ///    "reviewRequired",
    ///    "startDate",
    ///    "endDate",
    ///    "app",
    ///    "latestControlVersion",
    ///    "controlVersions",
    ///    "appStoreVersionExperimentTreatments"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppStoreVersionExperimentsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "trafficProportion")]
        TrafficProportion,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "reviewRequired")]
        ReviewRequired,
        #[serde(rename = "startDate")]
        StartDate,
        #[serde(rename = "endDate")]
        EndDate,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "latestControlVersion")]
        LatestControlVersion,
        #[serde(rename = "controlVersions")]
        ControlVersions,
        #[serde(rename = "appStoreVersionExperimentTreatments")]
        AppStoreVersionExperimentTreatments,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppStoreVersionExperimentsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::Platform => f.write_str("platform"),
                Self::TrafficProportion => f.write_str("trafficProportion"),
                Self::State => f.write_str("state"),
                Self::ReviewRequired => f.write_str("reviewRequired"),
                Self::StartDate => f.write_str("startDate"),
                Self::EndDate => f.write_str("endDate"),
                Self::App => f.write_str("app"),
                Self::LatestControlVersion => f.write_str("latestControlVersion"),
                Self::ControlVersions => f.write_str("controlVersions"),
                Self::AppStoreVersionExperimentTreatments => {
                    f.write_str("appStoreVersionExperimentTreatments")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppStoreVersionExperimentsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "platform" => Ok(Self::Platform),
                "trafficProportion" => Ok(Self::TrafficProportion),
                "state" => Ok(Self::State),
                "reviewRequired" => Ok(Self::ReviewRequired),
                "startDate" => Ok(Self::StartDate),
                "endDate" => Ok(Self::EndDate),
                "app" => Ok(Self::App),
                "latestControlVersion" => Ok(Self::LatestControlVersion),
                "controlVersions" => Ok(Self::ControlVersions),
                "appStoreVersionExperimentTreatments" => {
                    Ok(Self::AppStoreVersionExperimentTreatments)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppStoreVersionExperimentsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppStoreVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "versionString",
    ///    "appStoreState",
    ///    "appVersionState",
    ///    "copyright",
    ///    "reviewType",
    ///    "releaseType",
    ///    "earliestReleaseDate",
    ///    "usesIdfa",
    ///    "downloadable",
    ///    "createdDate",
    ///    "app",
    ///    "appStoreVersionLocalizations",
    ///    "build",
    ///    "appStoreVersionPhasedRelease",
    ///    "gameCenterAppVersion",
    ///    "routingAppCoverage",
    ///    "appStoreReviewDetail",
    ///    "appStoreVersionSubmission",
    ///    "appClipDefaultExperience",
    ///    "appStoreVersionExperiments",
    ///    "appStoreVersionExperimentsV2",
    ///    "customerReviews",
    ///    "alternativeDistributionPackage"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppStoreVersionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "versionString")]
        VersionString,
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "appVersionState")]
        AppVersionState,
        #[serde(rename = "copyright")]
        Copyright,
        #[serde(rename = "reviewType")]
        ReviewType,
        #[serde(rename = "releaseType")]
        ReleaseType,
        #[serde(rename = "earliestReleaseDate")]
        EarliestReleaseDate,
        #[serde(rename = "usesIdfa")]
        UsesIdfa,
        #[serde(rename = "downloadable")]
        Downloadable,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "appStoreVersionPhasedRelease")]
        AppStoreVersionPhasedRelease,
        #[serde(rename = "gameCenterAppVersion")]
        GameCenterAppVersion,
        #[serde(rename = "routingAppCoverage")]
        RoutingAppCoverage,
        #[serde(rename = "appStoreReviewDetail")]
        AppStoreReviewDetail,
        #[serde(rename = "appStoreVersionSubmission")]
        AppStoreVersionSubmission,
        #[serde(rename = "appClipDefaultExperience")]
        AppClipDefaultExperience,
        #[serde(rename = "appStoreVersionExperiments")]
        AppStoreVersionExperiments,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "alternativeDistributionPackage")]
        AlternativeDistributionPackage,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppStoreVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::VersionString => f.write_str("versionString"),
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::AppVersionState => f.write_str("appVersionState"),
                Self::Copyright => f.write_str("copyright"),
                Self::ReviewType => f.write_str("reviewType"),
                Self::ReleaseType => f.write_str("releaseType"),
                Self::EarliestReleaseDate => f.write_str("earliestReleaseDate"),
                Self::UsesIdfa => f.write_str("usesIdfa"),
                Self::Downloadable => f.write_str("downloadable"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::App => f.write_str("app"),
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
                Self::Build => f.write_str("build"),
                Self::AppStoreVersionPhasedRelease => f.write_str("appStoreVersionPhasedRelease"),
                Self::GameCenterAppVersion => f.write_str("gameCenterAppVersion"),
                Self::RoutingAppCoverage => f.write_str("routingAppCoverage"),
                Self::AppStoreReviewDetail => f.write_str("appStoreReviewDetail"),
                Self::AppStoreVersionSubmission => f.write_str("appStoreVersionSubmission"),
                Self::AppClipDefaultExperience => f.write_str("appClipDefaultExperience"),
                Self::AppStoreVersionExperiments => f.write_str("appStoreVersionExperiments"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::AlternativeDistributionPackage => {
                    f.write_str("alternativeDistributionPackage")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppStoreVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "versionString" => Ok(Self::VersionString),
                "appStoreState" => Ok(Self::AppStoreState),
                "appVersionState" => Ok(Self::AppVersionState),
                "copyright" => Ok(Self::Copyright),
                "reviewType" => Ok(Self::ReviewType),
                "releaseType" => Ok(Self::ReleaseType),
                "earliestReleaseDate" => Ok(Self::EarliestReleaseDate),
                "usesIdfa" => Ok(Self::UsesIdfa),
                "downloadable" => Ok(Self::Downloadable),
                "createdDate" => Ok(Self::CreatedDate),
                "app" => Ok(Self::App),
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                "build" => Ok(Self::Build),
                "appStoreVersionPhasedRelease" => Ok(Self::AppStoreVersionPhasedRelease),
                "gameCenterAppVersion" => Ok(Self::GameCenterAppVersion),
                "routingAppCoverage" => Ok(Self::RoutingAppCoverage),
                "appStoreReviewDetail" => Ok(Self::AppStoreReviewDetail),
                "appStoreVersionSubmission" => Ok(Self::AppStoreVersionSubmission),
                "appClipDefaultExperience" => Ok(Self::AppClipDefaultExperience),
                "appStoreVersionExperiments" => Ok(Self::AppStoreVersionExperiments),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "customerReviews" => Ok(Self::CustomerReviews),
                "alternativeDistributionPackage" => Ok(Self::AlternativeDistributionPackage),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppStoreVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "accessibilityUrl",
    ///    "name",
    ///    "bundleId",
    ///    "sku",
    ///    "primaryLocale",
    ///    "isOrEverWasMadeForKids",
    ///    "subscriptionStatusUrl",
    ///    "subscriptionStatusUrlVersion",
    ///    "subscriptionStatusUrlForSandbox",
    ///    "subscriptionStatusUrlVersionForSandbox",
    ///    "contentRightsDeclaration",
    ///    "streamlinedPurchasingEnabled",
    ///    "accessibilityDeclarations",
    ///    "appEncryptionDeclarations",
    ///    "appStoreIcon",
    ///    "ciProduct",
    ///    "betaTesters",
    ///    "betaGroups",
    ///    "appStoreVersions",
    ///    "appTags",
    ///    "preReleaseVersions",
    ///    "betaAppLocalizations",
    ///    "builds",
    ///    "betaLicenseAgreement",
    ///    "betaAppReviewDetail",
    ///    "appInfos",
    ///    "appClips",
    ///    "appPricePoints",
    ///    "endUserLicenseAgreement",
    ///    "appPriceSchedule",
    ///    "appAvailabilityV2",
    ///    "inAppPurchases",
    ///    "subscriptionGroups",
    ///    "gameCenterEnabledVersions",
    ///    "perfPowerMetrics",
    ///    "appCustomProductPages",
    ///    "inAppPurchasesV2",
    ///    "promotedPurchases",
    ///    "appEvents",
    ///    "reviewSubmissions",
    ///    "subscriptionGracePeriod",
    ///    "customerReviews",
    ///    "customerReviewSummarizations",
    ///    "gameCenterDetail",
    ///    "appStoreVersionExperimentsV2",
    ///    "alternativeDistributionKey",
    ///    "analyticsReportRequests",
    ///    "marketplaceSearchDetail",
    ///    "buildUploads",
    ///    "backgroundAssets",
    ///    "betaFeedbackScreenshotSubmissions",
    ///    "betaFeedbackCrashSubmissions",
    ///    "searchKeywords",
    ///    "webhooks",
    ///    "androidToIosAppMappingDetails"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppsItem {
        #[serde(rename = "accessibilityUrl")]
        AccessibilityUrl,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "sku")]
        Sku,
        #[serde(rename = "primaryLocale")]
        PrimaryLocale,
        #[serde(rename = "isOrEverWasMadeForKids")]
        IsOrEverWasMadeForKids,
        #[serde(rename = "subscriptionStatusUrl")]
        SubscriptionStatusUrl,
        #[serde(rename = "subscriptionStatusUrlVersion")]
        SubscriptionStatusUrlVersion,
        #[serde(rename = "subscriptionStatusUrlForSandbox")]
        SubscriptionStatusUrlForSandbox,
        #[serde(rename = "subscriptionStatusUrlVersionForSandbox")]
        SubscriptionStatusUrlVersionForSandbox,
        #[serde(rename = "contentRightsDeclaration")]
        ContentRightsDeclaration,
        #[serde(rename = "streamlinedPurchasingEnabled")]
        StreamlinedPurchasingEnabled,
        #[serde(rename = "accessibilityDeclarations")]
        AccessibilityDeclarations,
        #[serde(rename = "appEncryptionDeclarations")]
        AppEncryptionDeclarations,
        #[serde(rename = "appStoreIcon")]
        AppStoreIcon,
        #[serde(rename = "ciProduct")]
        CiProduct,
        #[serde(rename = "betaTesters")]
        BetaTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
        #[serde(rename = "appTags")]
        AppTags,
        #[serde(rename = "preReleaseVersions")]
        PreReleaseVersions,
        #[serde(rename = "betaAppLocalizations")]
        BetaAppLocalizations,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaLicenseAgreement")]
        BetaLicenseAgreement,
        #[serde(rename = "betaAppReviewDetail")]
        BetaAppReviewDetail,
        #[serde(rename = "appInfos")]
        AppInfos,
        #[serde(rename = "appClips")]
        AppClips,
        #[serde(rename = "appPricePoints")]
        AppPricePoints,
        #[serde(rename = "endUserLicenseAgreement")]
        EndUserLicenseAgreement,
        #[serde(rename = "appPriceSchedule")]
        AppPriceSchedule,
        #[serde(rename = "appAvailabilityV2")]
        AppAvailabilityV2,
        #[serde(rename = "inAppPurchases")]
        InAppPurchases,
        #[serde(rename = "subscriptionGroups")]
        SubscriptionGroups,
        #[serde(rename = "gameCenterEnabledVersions")]
        GameCenterEnabledVersions,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "appCustomProductPages")]
        AppCustomProductPages,
        #[serde(rename = "inAppPurchasesV2")]
        InAppPurchasesV2,
        #[serde(rename = "promotedPurchases")]
        PromotedPurchases,
        #[serde(rename = "appEvents")]
        AppEvents,
        #[serde(rename = "reviewSubmissions")]
        ReviewSubmissions,
        #[serde(rename = "subscriptionGracePeriod")]
        SubscriptionGracePeriod,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "customerReviewSummarizations")]
        CustomerReviewSummarizations,
        #[serde(rename = "gameCenterDetail")]
        GameCenterDetail,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "alternativeDistributionKey")]
        AlternativeDistributionKey,
        #[serde(rename = "analyticsReportRequests")]
        AnalyticsReportRequests,
        #[serde(rename = "marketplaceSearchDetail")]
        MarketplaceSearchDetail,
        #[serde(rename = "buildUploads")]
        BuildUploads,
        #[serde(rename = "backgroundAssets")]
        BackgroundAssets,
        #[serde(rename = "betaFeedbackScreenshotSubmissions")]
        BetaFeedbackScreenshotSubmissions,
        #[serde(rename = "betaFeedbackCrashSubmissions")]
        BetaFeedbackCrashSubmissions,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
        #[serde(rename = "webhooks")]
        Webhooks,
        #[serde(rename = "androidToIosAppMappingDetails")]
        AndroidToIosAppMappingDetails,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AccessibilityUrl => f.write_str("accessibilityUrl"),
                Self::Name => f.write_str("name"),
                Self::BundleId => f.write_str("bundleId"),
                Self::Sku => f.write_str("sku"),
                Self::PrimaryLocale => f.write_str("primaryLocale"),
                Self::IsOrEverWasMadeForKids => f.write_str("isOrEverWasMadeForKids"),
                Self::SubscriptionStatusUrl => f.write_str("subscriptionStatusUrl"),
                Self::SubscriptionStatusUrlVersion => f.write_str("subscriptionStatusUrlVersion"),
                Self::SubscriptionStatusUrlForSandbox => {
                    f.write_str("subscriptionStatusUrlForSandbox")
                }
                Self::SubscriptionStatusUrlVersionForSandbox => {
                    f.write_str("subscriptionStatusUrlVersionForSandbox")
                }
                Self::ContentRightsDeclaration => f.write_str("contentRightsDeclaration"),
                Self::StreamlinedPurchasingEnabled => f.write_str("streamlinedPurchasingEnabled"),
                Self::AccessibilityDeclarations => f.write_str("accessibilityDeclarations"),
                Self::AppEncryptionDeclarations => f.write_str("appEncryptionDeclarations"),
                Self::AppStoreIcon => f.write_str("appStoreIcon"),
                Self::CiProduct => f.write_str("ciProduct"),
                Self::BetaTesters => f.write_str("betaTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
                Self::AppTags => f.write_str("appTags"),
                Self::PreReleaseVersions => f.write_str("preReleaseVersions"),
                Self::BetaAppLocalizations => f.write_str("betaAppLocalizations"),
                Self::Builds => f.write_str("builds"),
                Self::BetaLicenseAgreement => f.write_str("betaLicenseAgreement"),
                Self::BetaAppReviewDetail => f.write_str("betaAppReviewDetail"),
                Self::AppInfos => f.write_str("appInfos"),
                Self::AppClips => f.write_str("appClips"),
                Self::AppPricePoints => f.write_str("appPricePoints"),
                Self::EndUserLicenseAgreement => f.write_str("endUserLicenseAgreement"),
                Self::AppPriceSchedule => f.write_str("appPriceSchedule"),
                Self::AppAvailabilityV2 => f.write_str("appAvailabilityV2"),
                Self::InAppPurchases => f.write_str("inAppPurchases"),
                Self::SubscriptionGroups => f.write_str("subscriptionGroups"),
                Self::GameCenterEnabledVersions => f.write_str("gameCenterEnabledVersions"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::AppCustomProductPages => f.write_str("appCustomProductPages"),
                Self::InAppPurchasesV2 => f.write_str("inAppPurchasesV2"),
                Self::PromotedPurchases => f.write_str("promotedPurchases"),
                Self::AppEvents => f.write_str("appEvents"),
                Self::ReviewSubmissions => f.write_str("reviewSubmissions"),
                Self::SubscriptionGracePeriod => f.write_str("subscriptionGracePeriod"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::CustomerReviewSummarizations => f.write_str("customerReviewSummarizations"),
                Self::GameCenterDetail => f.write_str("gameCenterDetail"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AlternativeDistributionKey => f.write_str("alternativeDistributionKey"),
                Self::AnalyticsReportRequests => f.write_str("analyticsReportRequests"),
                Self::MarketplaceSearchDetail => f.write_str("marketplaceSearchDetail"),
                Self::BuildUploads => f.write_str("buildUploads"),
                Self::BackgroundAssets => f.write_str("backgroundAssets"),
                Self::BetaFeedbackScreenshotSubmissions => {
                    f.write_str("betaFeedbackScreenshotSubmissions")
                }
                Self::BetaFeedbackCrashSubmissions => f.write_str("betaFeedbackCrashSubmissions"),
                Self::SearchKeywords => f.write_str("searchKeywords"),
                Self::Webhooks => f.write_str("webhooks"),
                Self::AndroidToIosAppMappingDetails => f.write_str("androidToIosAppMappingDetails"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "accessibilityUrl" => Ok(Self::AccessibilityUrl),
                "name" => Ok(Self::Name),
                "bundleId" => Ok(Self::BundleId),
                "sku" => Ok(Self::Sku),
                "primaryLocale" => Ok(Self::PrimaryLocale),
                "isOrEverWasMadeForKids" => Ok(Self::IsOrEverWasMadeForKids),
                "subscriptionStatusUrl" => Ok(Self::SubscriptionStatusUrl),
                "subscriptionStatusUrlVersion" => Ok(Self::SubscriptionStatusUrlVersion),
                "subscriptionStatusUrlForSandbox" => Ok(Self::SubscriptionStatusUrlForSandbox),
                "subscriptionStatusUrlVersionForSandbox" => {
                    Ok(Self::SubscriptionStatusUrlVersionForSandbox)
                }
                "contentRightsDeclaration" => Ok(Self::ContentRightsDeclaration),
                "streamlinedPurchasingEnabled" => Ok(Self::StreamlinedPurchasingEnabled),
                "accessibilityDeclarations" => Ok(Self::AccessibilityDeclarations),
                "appEncryptionDeclarations" => Ok(Self::AppEncryptionDeclarations),
                "appStoreIcon" => Ok(Self::AppStoreIcon),
                "ciProduct" => Ok(Self::CiProduct),
                "betaTesters" => Ok(Self::BetaTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                "appTags" => Ok(Self::AppTags),
                "preReleaseVersions" => Ok(Self::PreReleaseVersions),
                "betaAppLocalizations" => Ok(Self::BetaAppLocalizations),
                "builds" => Ok(Self::Builds),
                "betaLicenseAgreement" => Ok(Self::BetaLicenseAgreement),
                "betaAppReviewDetail" => Ok(Self::BetaAppReviewDetail),
                "appInfos" => Ok(Self::AppInfos),
                "appClips" => Ok(Self::AppClips),
                "appPricePoints" => Ok(Self::AppPricePoints),
                "endUserLicenseAgreement" => Ok(Self::EndUserLicenseAgreement),
                "appPriceSchedule" => Ok(Self::AppPriceSchedule),
                "appAvailabilityV2" => Ok(Self::AppAvailabilityV2),
                "inAppPurchases" => Ok(Self::InAppPurchases),
                "subscriptionGroups" => Ok(Self::SubscriptionGroups),
                "gameCenterEnabledVersions" => Ok(Self::GameCenterEnabledVersions),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "appCustomProductPages" => Ok(Self::AppCustomProductPages),
                "inAppPurchasesV2" => Ok(Self::InAppPurchasesV2),
                "promotedPurchases" => Ok(Self::PromotedPurchases),
                "appEvents" => Ok(Self::AppEvents),
                "reviewSubmissions" => Ok(Self::ReviewSubmissions),
                "subscriptionGracePeriod" => Ok(Self::SubscriptionGracePeriod),
                "customerReviews" => Ok(Self::CustomerReviews),
                "customerReviewSummarizations" => Ok(Self::CustomerReviewSummarizations),
                "gameCenterDetail" => Ok(Self::GameCenterDetail),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "alternativeDistributionKey" => Ok(Self::AlternativeDistributionKey),
                "analyticsReportRequests" => Ok(Self::AnalyticsReportRequests),
                "marketplaceSearchDetail" => Ok(Self::MarketplaceSearchDetail),
                "buildUploads" => Ok(Self::BuildUploads),
                "backgroundAssets" => Ok(Self::BackgroundAssets),
                "betaFeedbackScreenshotSubmissions" => Ok(Self::BetaFeedbackScreenshotSubmissions),
                "betaFeedbackCrashSubmissions" => Ok(Self::BetaFeedbackCrashSubmissions),
                "searchKeywords" => Ok(Self::SearchKeywords),
                "webhooks" => Ok(Self::Webhooks),
                "androidToIosAppMappingDetails" => Ok(Self::AndroidToIosAppMappingDetails),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsBetaAppLocalizationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "feedbackEmail",
    ///    "marketingUrl",
    ///    "privacyPolicyUrl",
    ///    "tvOsPrivacyPolicy",
    ///    "description",
    ///    "locale",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsBetaAppLocalizationsItem {
        #[serde(rename = "feedbackEmail")]
        FeedbackEmail,
        #[serde(rename = "marketingUrl")]
        MarketingUrl,
        #[serde(rename = "privacyPolicyUrl")]
        PrivacyPolicyUrl,
        #[serde(rename = "tvOsPrivacyPolicy")]
        TvOsPrivacyPolicy,
        #[serde(rename = "description")]
        Description,
        #[serde(rename = "locale")]
        Locale,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsBetaAppLocalizationsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::FeedbackEmail => f.write_str("feedbackEmail"),
                Self::MarketingUrl => f.write_str("marketingUrl"),
                Self::PrivacyPolicyUrl => f.write_str("privacyPolicyUrl"),
                Self::TvOsPrivacyPolicy => f.write_str("tvOsPrivacyPolicy"),
                Self::Description => f.write_str("description"),
                Self::Locale => f.write_str("locale"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsBetaAppLocalizationsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "feedbackEmail" => Ok(Self::FeedbackEmail),
                "marketingUrl" => Ok(Self::MarketingUrl),
                "privacyPolicyUrl" => Ok(Self::PrivacyPolicyUrl),
                "tvOsPrivacyPolicy" => Ok(Self::TvOsPrivacyPolicy),
                "description" => Ok(Self::Description),
                "locale" => Ok(Self::Locale),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsBetaAppLocalizationsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsBetaAppLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsBetaAppLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsBetaAppReviewDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "contactFirstName",
    ///    "contactLastName",
    ///    "contactPhone",
    ///    "contactEmail",
    ///    "demoAccountName",
    ///    "demoAccountPassword",
    ///    "demoAccountRequired",
    ///    "notes",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsBetaAppReviewDetailsItem {
        #[serde(rename = "contactFirstName")]
        ContactFirstName,
        #[serde(rename = "contactLastName")]
        ContactLastName,
        #[serde(rename = "contactPhone")]
        ContactPhone,
        #[serde(rename = "contactEmail")]
        ContactEmail,
        #[serde(rename = "demoAccountName")]
        DemoAccountName,
        #[serde(rename = "demoAccountPassword")]
        DemoAccountPassword,
        #[serde(rename = "demoAccountRequired")]
        DemoAccountRequired,
        #[serde(rename = "notes")]
        Notes,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsBetaAppReviewDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ContactFirstName => f.write_str("contactFirstName"),
                Self::ContactLastName => f.write_str("contactLastName"),
                Self::ContactPhone => f.write_str("contactPhone"),
                Self::ContactEmail => f.write_str("contactEmail"),
                Self::DemoAccountName => f.write_str("demoAccountName"),
                Self::DemoAccountPassword => f.write_str("demoAccountPassword"),
                Self::DemoAccountRequired => f.write_str("demoAccountRequired"),
                Self::Notes => f.write_str("notes"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsBetaAppReviewDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "contactFirstName" => Ok(Self::ContactFirstName),
                "contactLastName" => Ok(Self::ContactLastName),
                "contactPhone" => Ok(Self::ContactPhone),
                "contactEmail" => Ok(Self::ContactEmail),
                "demoAccountName" => Ok(Self::DemoAccountName),
                "demoAccountPassword" => Ok(Self::DemoAccountPassword),
                "demoAccountRequired" => Ok(Self::DemoAccountRequired),
                "notes" => Ok(Self::Notes),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsBetaAppReviewDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsBetaAppReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsBetaAppReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsBetaGroupsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "createdDate",
    ///    "isInternalGroup",
    ///    "hasAccessToAllBuilds",
    ///    "publicLinkEnabled",
    ///    "publicLinkId",
    ///    "publicLinkLimitEnabled",
    ///    "publicLinkLimit",
    ///    "publicLink",
    ///    "feedbackEnabled",
    ///    "iosBuildsAvailableForAppleSiliconMac",
    ///    "iosBuildsAvailableForAppleVision",
    ///    "app",
    ///    "builds",
    ///    "betaTesters",
    ///    "betaRecruitmentCriteria",
    ///    "betaRecruitmentCriterionCompatibleBuildCheck"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsBetaGroupsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "isInternalGroup")]
        IsInternalGroup,
        #[serde(rename = "hasAccessToAllBuilds")]
        HasAccessToAllBuilds,
        #[serde(rename = "publicLinkEnabled")]
        PublicLinkEnabled,
        #[serde(rename = "publicLinkId")]
        PublicLinkId,
        #[serde(rename = "publicLinkLimitEnabled")]
        PublicLinkLimitEnabled,
        #[serde(rename = "publicLinkLimit")]
        PublicLinkLimit,
        #[serde(rename = "publicLink")]
        PublicLink,
        #[serde(rename = "feedbackEnabled")]
        FeedbackEnabled,
        #[serde(rename = "iosBuildsAvailableForAppleSiliconMac")]
        IosBuildsAvailableForAppleSiliconMac,
        #[serde(rename = "iosBuildsAvailableForAppleVision")]
        IosBuildsAvailableForAppleVision,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaTesters")]
        BetaTesters,
        #[serde(rename = "betaRecruitmentCriteria")]
        BetaRecruitmentCriteria,
        #[serde(rename = "betaRecruitmentCriterionCompatibleBuildCheck")]
        BetaRecruitmentCriterionCompatibleBuildCheck,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsBetaGroupsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::IsInternalGroup => f.write_str("isInternalGroup"),
                Self::HasAccessToAllBuilds => f.write_str("hasAccessToAllBuilds"),
                Self::PublicLinkEnabled => f.write_str("publicLinkEnabled"),
                Self::PublicLinkId => f.write_str("publicLinkId"),
                Self::PublicLinkLimitEnabled => f.write_str("publicLinkLimitEnabled"),
                Self::PublicLinkLimit => f.write_str("publicLinkLimit"),
                Self::PublicLink => f.write_str("publicLink"),
                Self::FeedbackEnabled => f.write_str("feedbackEnabled"),
                Self::IosBuildsAvailableForAppleSiliconMac => {
                    f.write_str("iosBuildsAvailableForAppleSiliconMac")
                }
                Self::IosBuildsAvailableForAppleVision => {
                    f.write_str("iosBuildsAvailableForAppleVision")
                }
                Self::App => f.write_str("app"),
                Self::Builds => f.write_str("builds"),
                Self::BetaTesters => f.write_str("betaTesters"),
                Self::BetaRecruitmentCriteria => f.write_str("betaRecruitmentCriteria"),
                Self::BetaRecruitmentCriterionCompatibleBuildCheck => {
                    f.write_str("betaRecruitmentCriterionCompatibleBuildCheck")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsBetaGroupsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "createdDate" => Ok(Self::CreatedDate),
                "isInternalGroup" => Ok(Self::IsInternalGroup),
                "hasAccessToAllBuilds" => Ok(Self::HasAccessToAllBuilds),
                "publicLinkEnabled" => Ok(Self::PublicLinkEnabled),
                "publicLinkId" => Ok(Self::PublicLinkId),
                "publicLinkLimitEnabled" => Ok(Self::PublicLinkLimitEnabled),
                "publicLinkLimit" => Ok(Self::PublicLinkLimit),
                "publicLink" => Ok(Self::PublicLink),
                "feedbackEnabled" => Ok(Self::FeedbackEnabled),
                "iosBuildsAvailableForAppleSiliconMac" => {
                    Ok(Self::IosBuildsAvailableForAppleSiliconMac)
                }
                "iosBuildsAvailableForAppleVision" => Ok(Self::IosBuildsAvailableForAppleVision),
                "app" => Ok(Self::App),
                "builds" => Ok(Self::Builds),
                "betaTesters" => Ok(Self::BetaTesters),
                "betaRecruitmentCriteria" => Ok(Self::BetaRecruitmentCriteria),
                "betaRecruitmentCriterionCompatibleBuildCheck" => {
                    Ok(Self::BetaRecruitmentCriterionCompatibleBuildCheck)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsBetaGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsBetaGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsBetaGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsBetaLicenseAgreementsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "agreementText",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsBetaLicenseAgreementsItem {
        #[serde(rename = "agreementText")]
        AgreementText,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsBetaLicenseAgreementsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AgreementText => f.write_str("agreementText"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsBetaLicenseAgreementsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "agreementText" => Ok(Self::AgreementText),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsBetaLicenseAgreementsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsBetaLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsBetaLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsBuildIconsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "iconAsset",
    ///    "iconType",
    ///    "masked",
    ///    "name"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsBuildIconsItem {
        #[serde(rename = "iconAsset")]
        IconAsset,
        #[serde(rename = "iconType")]
        IconType,
        #[serde(rename = "masked")]
        Masked,
        #[serde(rename = "name")]
        Name,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsBuildIconsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::IconAsset => f.write_str("iconAsset"),
                Self::IconType => f.write_str("iconType"),
                Self::Masked => f.write_str("masked"),
                Self::Name => f.write_str("name"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsBuildIconsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "iconAsset" => Ok(Self::IconAsset),
                "iconType" => Ok(Self::IconType),
                "masked" => Ok(Self::Masked),
                "name" => Ok(Self::Name),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsBuildIconsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsBuildIconsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsBuildIconsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsBuildsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "version",
    ///    "uploadedDate",
    ///    "expirationDate",
    ///    "expired",
    ///    "minOsVersion",
    ///    "lsMinimumSystemVersion",
    ///    "computedMinMacOsVersion",
    ///    "computedMinVisionOsVersion",
    ///    "iconAssetToken",
    ///    "processingState",
    ///    "buildAudienceType",
    ///    "usesNonExemptEncryption",
    ///    "preReleaseVersion",
    ///    "individualTesters",
    ///    "betaGroups",
    ///    "betaBuildLocalizations",
    ///    "appEncryptionDeclaration",
    ///    "betaAppReviewSubmission",
    ///    "app",
    ///    "buildBetaDetail",
    ///    "appStoreVersion",
    ///    "icons",
    ///    "buildBundles",
    ///    "buildUpload",
    ///    "perfPowerMetrics",
    ///    "diagnosticSignatures"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsBuildsItem {
        #[serde(rename = "version")]
        Version,
        #[serde(rename = "uploadedDate")]
        UploadedDate,
        #[serde(rename = "expirationDate")]
        ExpirationDate,
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "minOsVersion")]
        MinOsVersion,
        #[serde(rename = "lsMinimumSystemVersion")]
        LsMinimumSystemVersion,
        #[serde(rename = "computedMinMacOsVersion")]
        ComputedMinMacOsVersion,
        #[serde(rename = "computedMinVisionOsVersion")]
        ComputedMinVisionOsVersion,
        #[serde(rename = "iconAssetToken")]
        IconAssetToken,
        #[serde(rename = "processingState")]
        ProcessingState,
        #[serde(rename = "buildAudienceType")]
        BuildAudienceType,
        #[serde(rename = "usesNonExemptEncryption")]
        UsesNonExemptEncryption,
        #[serde(rename = "preReleaseVersion")]
        PreReleaseVersion,
        #[serde(rename = "individualTesters")]
        IndividualTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "betaBuildLocalizations")]
        BetaBuildLocalizations,
        #[serde(rename = "appEncryptionDeclaration")]
        AppEncryptionDeclaration,
        #[serde(rename = "betaAppReviewSubmission")]
        BetaAppReviewSubmission,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "buildBetaDetail")]
        BuildBetaDetail,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "icons")]
        Icons,
        #[serde(rename = "buildBundles")]
        BuildBundles,
        #[serde(rename = "buildUpload")]
        BuildUpload,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "diagnosticSignatures")]
        DiagnosticSignatures,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsBuildsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Version => f.write_str("version"),
                Self::UploadedDate => f.write_str("uploadedDate"),
                Self::ExpirationDate => f.write_str("expirationDate"),
                Self::Expired => f.write_str("expired"),
                Self::MinOsVersion => f.write_str("minOsVersion"),
                Self::LsMinimumSystemVersion => f.write_str("lsMinimumSystemVersion"),
                Self::ComputedMinMacOsVersion => f.write_str("computedMinMacOsVersion"),
                Self::ComputedMinVisionOsVersion => f.write_str("computedMinVisionOsVersion"),
                Self::IconAssetToken => f.write_str("iconAssetToken"),
                Self::ProcessingState => f.write_str("processingState"),
                Self::BuildAudienceType => f.write_str("buildAudienceType"),
                Self::UsesNonExemptEncryption => f.write_str("usesNonExemptEncryption"),
                Self::PreReleaseVersion => f.write_str("preReleaseVersion"),
                Self::IndividualTesters => f.write_str("individualTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::BetaBuildLocalizations => f.write_str("betaBuildLocalizations"),
                Self::AppEncryptionDeclaration => f.write_str("appEncryptionDeclaration"),
                Self::BetaAppReviewSubmission => f.write_str("betaAppReviewSubmission"),
                Self::App => f.write_str("app"),
                Self::BuildBetaDetail => f.write_str("buildBetaDetail"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::Icons => f.write_str("icons"),
                Self::BuildBundles => f.write_str("buildBundles"),
                Self::BuildUpload => f.write_str("buildUpload"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::DiagnosticSignatures => f.write_str("diagnosticSignatures"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsBuildsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "version" => Ok(Self::Version),
                "uploadedDate" => Ok(Self::UploadedDate),
                "expirationDate" => Ok(Self::ExpirationDate),
                "expired" => Ok(Self::Expired),
                "minOsVersion" => Ok(Self::MinOsVersion),
                "lsMinimumSystemVersion" => Ok(Self::LsMinimumSystemVersion),
                "computedMinMacOsVersion" => Ok(Self::ComputedMinMacOsVersion),
                "computedMinVisionOsVersion" => Ok(Self::ComputedMinVisionOsVersion),
                "iconAssetToken" => Ok(Self::IconAssetToken),
                "processingState" => Ok(Self::ProcessingState),
                "buildAudienceType" => Ok(Self::BuildAudienceType),
                "usesNonExemptEncryption" => Ok(Self::UsesNonExemptEncryption),
                "preReleaseVersion" => Ok(Self::PreReleaseVersion),
                "individualTesters" => Ok(Self::IndividualTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "betaBuildLocalizations" => Ok(Self::BetaBuildLocalizations),
                "appEncryptionDeclaration" => Ok(Self::AppEncryptionDeclaration),
                "betaAppReviewSubmission" => Ok(Self::BetaAppReviewSubmission),
                "app" => Ok(Self::App),
                "buildBetaDetail" => Ok(Self::BuildBetaDetail),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "icons" => Ok(Self::Icons),
                "buildBundles" => Ok(Self::BuildBundles),
                "buildUpload" => Ok(Self::BuildUpload),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "diagnosticSignatures" => Ok(Self::DiagnosticSignatures),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsCiProductsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "createdDate",
    ///    "productType",
    ///    "app",
    ///    "bundleId",
    ///    "workflows",
    ///    "primaryRepositories",
    ///    "additionalRepositories",
    ///    "buildRuns"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsCiProductsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "productType")]
        ProductType,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "workflows")]
        Workflows,
        #[serde(rename = "primaryRepositories")]
        PrimaryRepositories,
        #[serde(rename = "additionalRepositories")]
        AdditionalRepositories,
        #[serde(rename = "buildRuns")]
        BuildRuns,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsCiProductsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::ProductType => f.write_str("productType"),
                Self::App => f.write_str("app"),
                Self::BundleId => f.write_str("bundleId"),
                Self::Workflows => f.write_str("workflows"),
                Self::PrimaryRepositories => f.write_str("primaryRepositories"),
                Self::AdditionalRepositories => f.write_str("additionalRepositories"),
                Self::BuildRuns => f.write_str("buildRuns"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsCiProductsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "createdDate" => Ok(Self::CreatedDate),
                "productType" => Ok(Self::ProductType),
                "app" => Ok(Self::App),
                "bundleId" => Ok(Self::BundleId),
                "workflows" => Ok(Self::Workflows),
                "primaryRepositories" => Ok(Self::PrimaryRepositories),
                "additionalRepositories" => Ok(Self::AdditionalRepositories),
                "buildRuns" => Ok(Self::BuildRuns),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsCiProductsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsCiProductsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsCiProductsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsEndUserLicenseAgreementsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "agreementText",
    ///    "app",
    ///    "territories"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsEndUserLicenseAgreementsItem {
        #[serde(rename = "agreementText")]
        AgreementText,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "territories")]
        Territories,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsEndUserLicenseAgreementsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AgreementText => f.write_str("agreementText"),
                Self::App => f.write_str("app"),
                Self::Territories => f.write_str("territories"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsEndUserLicenseAgreementsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "agreementText" => Ok(Self::AgreementText),
                "app" => Ok(Self::App),
                "territories" => Ok(Self::Territories),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsEndUserLicenseAgreementsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsEndUserLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsEndUserLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsGameCenterDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "arcadeEnabled",
    ///    "challengeEnabled",
    ///    "app",
    ///    "gameCenterAppVersions",
    ///    "gameCenterGroup",
    ///    "gameCenterLeaderboards",
    ///    "gameCenterLeaderboardsV2",
    ///    "gameCenterLeaderboardSets",
    ///    "gameCenterLeaderboardSetsV2",
    ///    "gameCenterAchievements",
    ///    "gameCenterAchievementsV2",
    ///    "gameCenterActivities",
    ///    "gameCenterChallenges",
    ///    "defaultLeaderboard",
    ///    "defaultLeaderboardV2",
    ///    "defaultGroupLeaderboard",
    ///    "defaultGroupLeaderboardV2",
    ///    "achievementReleases",
    ///    "activityReleases",
    ///    "challengeReleases",
    ///    "leaderboardReleases",
    ///    "leaderboardSetReleases",
    ///    "challengesMinimumPlatformVersions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsGameCenterDetailsItem {
        #[serde(rename = "arcadeEnabled")]
        ArcadeEnabled,
        #[serde(rename = "challengeEnabled")]
        ChallengeEnabled,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "gameCenterAppVersions")]
        GameCenterAppVersions,
        #[serde(rename = "gameCenterGroup")]
        GameCenterGroup,
        #[serde(rename = "gameCenterLeaderboards")]
        GameCenterLeaderboards,
        #[serde(rename = "gameCenterLeaderboardsV2")]
        GameCenterLeaderboardsV2,
        #[serde(rename = "gameCenterLeaderboardSets")]
        GameCenterLeaderboardSets,
        #[serde(rename = "gameCenterLeaderboardSetsV2")]
        GameCenterLeaderboardSetsV2,
        #[serde(rename = "gameCenterAchievements")]
        GameCenterAchievements,
        #[serde(rename = "gameCenterAchievementsV2")]
        GameCenterAchievementsV2,
        #[serde(rename = "gameCenterActivities")]
        GameCenterActivities,
        #[serde(rename = "gameCenterChallenges")]
        GameCenterChallenges,
        #[serde(rename = "defaultLeaderboard")]
        DefaultLeaderboard,
        #[serde(rename = "defaultLeaderboardV2")]
        DefaultLeaderboardV2,
        #[serde(rename = "defaultGroupLeaderboard")]
        DefaultGroupLeaderboard,
        #[serde(rename = "defaultGroupLeaderboardV2")]
        DefaultGroupLeaderboardV2,
        #[serde(rename = "achievementReleases")]
        AchievementReleases,
        #[serde(rename = "activityReleases")]
        ActivityReleases,
        #[serde(rename = "challengeReleases")]
        ChallengeReleases,
        #[serde(rename = "leaderboardReleases")]
        LeaderboardReleases,
        #[serde(rename = "leaderboardSetReleases")]
        LeaderboardSetReleases,
        #[serde(rename = "challengesMinimumPlatformVersions")]
        ChallengesMinimumPlatformVersions,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsGameCenterDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ArcadeEnabled => f.write_str("arcadeEnabled"),
                Self::ChallengeEnabled => f.write_str("challengeEnabled"),
                Self::App => f.write_str("app"),
                Self::GameCenterAppVersions => f.write_str("gameCenterAppVersions"),
                Self::GameCenterGroup => f.write_str("gameCenterGroup"),
                Self::GameCenterLeaderboards => f.write_str("gameCenterLeaderboards"),
                Self::GameCenterLeaderboardsV2 => f.write_str("gameCenterLeaderboardsV2"),
                Self::GameCenterLeaderboardSets => f.write_str("gameCenterLeaderboardSets"),
                Self::GameCenterLeaderboardSetsV2 => f.write_str("gameCenterLeaderboardSetsV2"),
                Self::GameCenterAchievements => f.write_str("gameCenterAchievements"),
                Self::GameCenterAchievementsV2 => f.write_str("gameCenterAchievementsV2"),
                Self::GameCenterActivities => f.write_str("gameCenterActivities"),
                Self::GameCenterChallenges => f.write_str("gameCenterChallenges"),
                Self::DefaultLeaderboard => f.write_str("defaultLeaderboard"),
                Self::DefaultLeaderboardV2 => f.write_str("defaultLeaderboardV2"),
                Self::DefaultGroupLeaderboard => f.write_str("defaultGroupLeaderboard"),
                Self::DefaultGroupLeaderboardV2 => f.write_str("defaultGroupLeaderboardV2"),
                Self::AchievementReleases => f.write_str("achievementReleases"),
                Self::ActivityReleases => f.write_str("activityReleases"),
                Self::ChallengeReleases => f.write_str("challengeReleases"),
                Self::LeaderboardReleases => f.write_str("leaderboardReleases"),
                Self::LeaderboardSetReleases => f.write_str("leaderboardSetReleases"),
                Self::ChallengesMinimumPlatformVersions => {
                    f.write_str("challengesMinimumPlatformVersions")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsGameCenterDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "arcadeEnabled" => Ok(Self::ArcadeEnabled),
                "challengeEnabled" => Ok(Self::ChallengeEnabled),
                "app" => Ok(Self::App),
                "gameCenterAppVersions" => Ok(Self::GameCenterAppVersions),
                "gameCenterGroup" => Ok(Self::GameCenterGroup),
                "gameCenterLeaderboards" => Ok(Self::GameCenterLeaderboards),
                "gameCenterLeaderboardsV2" => Ok(Self::GameCenterLeaderboardsV2),
                "gameCenterLeaderboardSets" => Ok(Self::GameCenterLeaderboardSets),
                "gameCenterLeaderboardSetsV2" => Ok(Self::GameCenterLeaderboardSetsV2),
                "gameCenterAchievements" => Ok(Self::GameCenterAchievements),
                "gameCenterAchievementsV2" => Ok(Self::GameCenterAchievementsV2),
                "gameCenterActivities" => Ok(Self::GameCenterActivities),
                "gameCenterChallenges" => Ok(Self::GameCenterChallenges),
                "defaultLeaderboard" => Ok(Self::DefaultLeaderboard),
                "defaultLeaderboardV2" => Ok(Self::DefaultLeaderboardV2),
                "defaultGroupLeaderboard" => Ok(Self::DefaultGroupLeaderboard),
                "defaultGroupLeaderboardV2" => Ok(Self::DefaultGroupLeaderboardV2),
                "achievementReleases" => Ok(Self::AchievementReleases),
                "activityReleases" => Ok(Self::ActivityReleases),
                "challengeReleases" => Ok(Self::ChallengeReleases),
                "leaderboardReleases" => Ok(Self::LeaderboardReleases),
                "leaderboardSetReleases" => Ok(Self::LeaderboardSetReleases),
                "challengesMinimumPlatformVersions" => Ok(Self::ChallengesMinimumPlatformVersions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsGameCenterDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsGameCenterDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsGameCenterDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsGameCenterEnabledVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "versionString",
    ///    "iconAsset",
    ///    "compatibleVersions",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsGameCenterEnabledVersionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "versionString")]
        VersionString,
        #[serde(rename = "iconAsset")]
        IconAsset,
        #[serde(rename = "compatibleVersions")]
        CompatibleVersions,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsGameCenterEnabledVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::VersionString => f.write_str("versionString"),
                Self::IconAsset => f.write_str("iconAsset"),
                Self::CompatibleVersions => f.write_str("compatibleVersions"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsGameCenterEnabledVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "versionString" => Ok(Self::VersionString),
                "iconAsset" => Ok(Self::IconAsset),
                "compatibleVersions" => Ok(Self::CompatibleVersions),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsGameCenterEnabledVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsGameCenterEnabledVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsGameCenterEnabledVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsInAppPurchasesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "referenceName",
    ///    "productId",
    ///    "inAppPurchaseType",
    ///    "state",
    ///    "apps",
    ///    "name",
    ///    "reviewNote",
    ///    "familySharable",
    ///    "contentHosting",
    ///    "inAppPurchaseLocalizations",
    ///    "pricePoints",
    ///    "content",
    ///    "appStoreReviewScreenshot",
    ///    "promotedPurchase",
    ///    "iapPriceSchedule",
    ///    "inAppPurchaseAvailability",
    ///    "images",
    ///    "offerCodes"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsInAppPurchasesItem {
        #[serde(rename = "referenceName")]
        ReferenceName,
        #[serde(rename = "productId")]
        ProductId,
        #[serde(rename = "inAppPurchaseType")]
        InAppPurchaseType,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "apps")]
        Apps,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "reviewNote")]
        ReviewNote,
        #[serde(rename = "familySharable")]
        FamilySharable,
        #[serde(rename = "contentHosting")]
        ContentHosting,
        #[serde(rename = "inAppPurchaseLocalizations")]
        InAppPurchaseLocalizations,
        #[serde(rename = "pricePoints")]
        PricePoints,
        #[serde(rename = "content")]
        Content,
        #[serde(rename = "appStoreReviewScreenshot")]
        AppStoreReviewScreenshot,
        #[serde(rename = "promotedPurchase")]
        PromotedPurchase,
        #[serde(rename = "iapPriceSchedule")]
        IapPriceSchedule,
        #[serde(rename = "inAppPurchaseAvailability")]
        InAppPurchaseAvailability,
        #[serde(rename = "images")]
        Images,
        #[serde(rename = "offerCodes")]
        OfferCodes,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsInAppPurchasesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReferenceName => f.write_str("referenceName"),
                Self::ProductId => f.write_str("productId"),
                Self::InAppPurchaseType => f.write_str("inAppPurchaseType"),
                Self::State => f.write_str("state"),
                Self::Apps => f.write_str("apps"),
                Self::Name => f.write_str("name"),
                Self::ReviewNote => f.write_str("reviewNote"),
                Self::FamilySharable => f.write_str("familySharable"),
                Self::ContentHosting => f.write_str("contentHosting"),
                Self::InAppPurchaseLocalizations => f.write_str("inAppPurchaseLocalizations"),
                Self::PricePoints => f.write_str("pricePoints"),
                Self::Content => f.write_str("content"),
                Self::AppStoreReviewScreenshot => f.write_str("appStoreReviewScreenshot"),
                Self::PromotedPurchase => f.write_str("promotedPurchase"),
                Self::IapPriceSchedule => f.write_str("iapPriceSchedule"),
                Self::InAppPurchaseAvailability => f.write_str("inAppPurchaseAvailability"),
                Self::Images => f.write_str("images"),
                Self::OfferCodes => f.write_str("offerCodes"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsInAppPurchasesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "referenceName" => Ok(Self::ReferenceName),
                "productId" => Ok(Self::ProductId),
                "inAppPurchaseType" => Ok(Self::InAppPurchaseType),
                "state" => Ok(Self::State),
                "apps" => Ok(Self::Apps),
                "name" => Ok(Self::Name),
                "reviewNote" => Ok(Self::ReviewNote),
                "familySharable" => Ok(Self::FamilySharable),
                "contentHosting" => Ok(Self::ContentHosting),
                "inAppPurchaseLocalizations" => Ok(Self::InAppPurchaseLocalizations),
                "pricePoints" => Ok(Self::PricePoints),
                "content" => Ok(Self::Content),
                "appStoreReviewScreenshot" => Ok(Self::AppStoreReviewScreenshot),
                "promotedPurchase" => Ok(Self::PromotedPurchase),
                "iapPriceSchedule" => Ok(Self::IapPriceSchedule),
                "inAppPurchaseAvailability" => Ok(Self::InAppPurchaseAvailability),
                "images" => Ok(Self::Images),
                "offerCodes" => Ok(Self::OfferCodes),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsInAppPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsInAppPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsInAppPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsPreReleaseVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "version",
    ///    "platform",
    ///    "builds",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsPreReleaseVersionsItem {
        #[serde(rename = "version")]
        Version,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsPreReleaseVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Version => f.write_str("version"),
                Self::Platform => f.write_str("platform"),
                Self::Builds => f.write_str("builds"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsPreReleaseVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "version" => Ok(Self::Version),
                "platform" => Ok(Self::Platform),
                "builds" => Ok(Self::Builds),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsPreReleaseVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsPreReleaseVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsPreReleaseVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsPromotedPurchasesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "visibleForAllUsers",
    ///    "enabled",
    ///    "state",
    ///    "inAppPurchaseV2",
    ///    "subscription"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsPromotedPurchasesItem {
        #[serde(rename = "visibleForAllUsers")]
        VisibleForAllUsers,
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "inAppPurchaseV2")]
        InAppPurchaseV2,
        #[serde(rename = "subscription")]
        Subscription,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsPromotedPurchasesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::VisibleForAllUsers => f.write_str("visibleForAllUsers"),
                Self::Enabled => f.write_str("enabled"),
                Self::State => f.write_str("state"),
                Self::InAppPurchaseV2 => f.write_str("inAppPurchaseV2"),
                Self::Subscription => f.write_str("subscription"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsPromotedPurchasesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "visibleForAllUsers" => Ok(Self::VisibleForAllUsers),
                "enabled" => Ok(Self::Enabled),
                "state" => Ok(Self::State),
                "inAppPurchaseV2" => Ok(Self::InAppPurchaseV2),
                "subscription" => Ok(Self::Subscription),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsPromotedPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsPromotedPurchasesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsPromotedPurchasesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsReviewSubmissionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "submittedDate",
    ///    "state",
    ///    "app",
    ///    "items",
    ///    "appStoreVersionForReview",
    ///    "submittedByActor",
    ///    "lastUpdatedByActor"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsReviewSubmissionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "submittedDate")]
        SubmittedDate,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "items")]
        Items,
        #[serde(rename = "appStoreVersionForReview")]
        AppStoreVersionForReview,
        #[serde(rename = "submittedByActor")]
        SubmittedByActor,
        #[serde(rename = "lastUpdatedByActor")]
        LastUpdatedByActor,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsReviewSubmissionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::SubmittedDate => f.write_str("submittedDate"),
                Self::State => f.write_str("state"),
                Self::App => f.write_str("app"),
                Self::Items => f.write_str("items"),
                Self::AppStoreVersionForReview => f.write_str("appStoreVersionForReview"),
                Self::SubmittedByActor => f.write_str("submittedByActor"),
                Self::LastUpdatedByActor => f.write_str("lastUpdatedByActor"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsReviewSubmissionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "submittedDate" => Ok(Self::SubmittedDate),
                "state" => Ok(Self::State),
                "app" => Ok(Self::App),
                "items" => Ok(Self::Items),
                "appStoreVersionForReview" => Ok(Self::AppStoreVersionForReview),
                "submittedByActor" => Ok(Self::SubmittedByActor),
                "lastUpdatedByActor" => Ok(Self::LastUpdatedByActor),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsReviewSubmissionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsReviewSubmissionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsReviewSubmissionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsSubscriptionGracePeriodsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "optIn",
    ///    "sandboxOptIn",
    ///    "duration",
    ///    "renewalType"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsSubscriptionGracePeriodsItem {
        #[serde(rename = "optIn")]
        OptIn,
        #[serde(rename = "sandboxOptIn")]
        SandboxOptIn,
        #[serde(rename = "duration")]
        Duration,
        #[serde(rename = "renewalType")]
        RenewalType,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsSubscriptionGracePeriodsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::OptIn => f.write_str("optIn"),
                Self::SandboxOptIn => f.write_str("sandboxOptIn"),
                Self::Duration => f.write_str("duration"),
                Self::RenewalType => f.write_str("renewalType"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsSubscriptionGracePeriodsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "optIn" => Ok(Self::OptIn),
                "sandboxOptIn" => Ok(Self::SandboxOptIn),
                "duration" => Ok(Self::Duration),
                "renewalType" => Ok(Self::RenewalType),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsSubscriptionGracePeriodsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsSubscriptionGracePeriodsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsSubscriptionGracePeriodsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsSubscriptionGroupsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "referenceName",
    ///    "subscriptions",
    ///    "subscriptionGroupLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsSubscriptionGroupsItem {
        #[serde(rename = "referenceName")]
        ReferenceName,
        #[serde(rename = "subscriptions")]
        Subscriptions,
        #[serde(rename = "subscriptionGroupLocalizations")]
        SubscriptionGroupLocalizations,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsSubscriptionGroupsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReferenceName => f.write_str("referenceName"),
                Self::Subscriptions => f.write_str("subscriptions"),
                Self::SubscriptionGroupLocalizations => {
                    f.write_str("subscriptionGroupLocalizations")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsSubscriptionGroupsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "referenceName" => Ok(Self::ReferenceName),
                "subscriptions" => Ok(Self::Subscriptions),
                "subscriptionGroupLocalizations" => Ok(Self::SubscriptionGroupLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsSubscriptionGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsSubscriptionGroupsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsSubscriptionGroupsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACCEPTED",
    ///    "DEVELOPER_REMOVED_FROM_SALE",
    ///    "DEVELOPER_REJECTED",
    ///    "IN_REVIEW",
    ///    "INVALID_BINARY",
    ///    "METADATA_REJECTED",
    ///    "PENDING_APPLE_RELEASE",
    ///    "PENDING_CONTRACT",
    ///    "PENDING_DEVELOPER_RELEASE",
    ///    "PREPARE_FOR_SUBMISSION",
    ///    "PREORDER_READY_FOR_SALE",
    ///    "PROCESSING_FOR_APP_STORE",
    ///    "READY_FOR_REVIEW",
    ///    "READY_FOR_SALE",
    ///    "REJECTED",
    ///    "REMOVED_FROM_SALE",
    ///    "WAITING_FOR_EXPORT_COMPLIANCE",
    ///    "WAITING_FOR_REVIEW",
    ///    "REPLACED_WITH_NEW_VERSION",
    ///    "NOT_APPLICABLE"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem {
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "DEVELOPER_REMOVED_FROM_SALE")]
        DeveloperRemovedFromSale,
        #[serde(rename = "DEVELOPER_REJECTED")]
        DeveloperRejected,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "INVALID_BINARY")]
        InvalidBinary,
        #[serde(rename = "METADATA_REJECTED")]
        MetadataRejected,
        #[serde(rename = "PENDING_APPLE_RELEASE")]
        PendingAppleRelease,
        #[serde(rename = "PENDING_CONTRACT")]
        PendingContract,
        #[serde(rename = "PENDING_DEVELOPER_RELEASE")]
        PendingDeveloperRelease,
        #[serde(rename = "PREPARE_FOR_SUBMISSION")]
        PrepareForSubmission,
        #[serde(rename = "PREORDER_READY_FOR_SALE")]
        PreorderReadyForSale,
        #[serde(rename = "PROCESSING_FOR_APP_STORE")]
        ProcessingForAppStore,
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "READY_FOR_SALE")]
        ReadyForSale,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "REMOVED_FROM_SALE")]
        RemovedFromSale,
        #[serde(rename = "WAITING_FOR_EXPORT_COMPLIANCE")]
        WaitingForExportCompliance,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
        #[serde(rename = "REPLACED_WITH_NEW_VERSION")]
        ReplacedWithNewVersion,
        #[serde(rename = "NOT_APPLICABLE")]
        NotApplicable,
    }

    impl ::std::fmt::Display for AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::DeveloperRemovedFromSale => f.write_str("DEVELOPER_REMOVED_FROM_SALE"),
                Self::DeveloperRejected => f.write_str("DEVELOPER_REJECTED"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::InvalidBinary => f.write_str("INVALID_BINARY"),
                Self::MetadataRejected => f.write_str("METADATA_REJECTED"),
                Self::PendingAppleRelease => f.write_str("PENDING_APPLE_RELEASE"),
                Self::PendingContract => f.write_str("PENDING_CONTRACT"),
                Self::PendingDeveloperRelease => f.write_str("PENDING_DEVELOPER_RELEASE"),
                Self::PrepareForSubmission => f.write_str("PREPARE_FOR_SUBMISSION"),
                Self::PreorderReadyForSale => f.write_str("PREORDER_READY_FOR_SALE"),
                Self::ProcessingForAppStore => f.write_str("PROCESSING_FOR_APP_STORE"),
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::ReadyForSale => f.write_str("READY_FOR_SALE"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::RemovedFromSale => f.write_str("REMOVED_FROM_SALE"),
                Self::WaitingForExportCompliance => f.write_str("WAITING_FOR_EXPORT_COMPLIANCE"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
                Self::ReplacedWithNewVersion => f.write_str("REPLACED_WITH_NEW_VERSION"),
                Self::NotApplicable => f.write_str("NOT_APPLICABLE"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACCEPTED" => Ok(Self::Accepted),
                "DEVELOPER_REMOVED_FROM_SALE" => Ok(Self::DeveloperRemovedFromSale),
                "DEVELOPER_REJECTED" => Ok(Self::DeveloperRejected),
                "IN_REVIEW" => Ok(Self::InReview),
                "INVALID_BINARY" => Ok(Self::InvalidBinary),
                "METADATA_REJECTED" => Ok(Self::MetadataRejected),
                "PENDING_APPLE_RELEASE" => Ok(Self::PendingAppleRelease),
                "PENDING_CONTRACT" => Ok(Self::PendingContract),
                "PENDING_DEVELOPER_RELEASE" => Ok(Self::PendingDeveloperRelease),
                "PREPARE_FOR_SUBMISSION" => Ok(Self::PrepareForSubmission),
                "PREORDER_READY_FOR_SALE" => Ok(Self::PreorderReadyForSale),
                "PROCESSING_FOR_APP_STORE" => Ok(Self::ProcessingForAppStore),
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "READY_FOR_SALE" => Ok(Self::ReadyForSale),
                "REJECTED" => Ok(Self::Rejected),
                "REMOVED_FROM_SALE" => Ok(Self::RemovedFromSale),
                "WAITING_FOR_EXPORT_COMPLIANCE" => Ok(Self::WaitingForExportCompliance),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                "REPLACED_WITH_NEW_VERSION" => Ok(Self::ReplacedWithNewVersion),
                "NOT_APPLICABLE" => Ok(Self::NotApplicable),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACCEPTED",
    ///    "DEVELOPER_REJECTED",
    ///    "IN_REVIEW",
    ///    "INVALID_BINARY",
    ///    "METADATA_REJECTED",
    ///    "PENDING_APPLE_RELEASE",
    ///    "PENDING_DEVELOPER_RELEASE",
    ///    "PREPARE_FOR_SUBMISSION",
    ///    "PROCESSING_FOR_DISTRIBUTION",
    ///    "READY_FOR_DISTRIBUTION",
    ///    "READY_FOR_REVIEW",
    ///    "REJECTED",
    ///    "REPLACED_WITH_NEW_VERSION",
    ///    "WAITING_FOR_EXPORT_COMPLIANCE",
    ///    "WAITING_FOR_REVIEW"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem {
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "DEVELOPER_REJECTED")]
        DeveloperRejected,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "INVALID_BINARY")]
        InvalidBinary,
        #[serde(rename = "METADATA_REJECTED")]
        MetadataRejected,
        #[serde(rename = "PENDING_APPLE_RELEASE")]
        PendingAppleRelease,
        #[serde(rename = "PENDING_DEVELOPER_RELEASE")]
        PendingDeveloperRelease,
        #[serde(rename = "PREPARE_FOR_SUBMISSION")]
        PrepareForSubmission,
        #[serde(rename = "PROCESSING_FOR_DISTRIBUTION")]
        ProcessingForDistribution,
        #[serde(rename = "READY_FOR_DISTRIBUTION")]
        ReadyForDistribution,
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "REPLACED_WITH_NEW_VERSION")]
        ReplacedWithNewVersion,
        #[serde(rename = "WAITING_FOR_EXPORT_COMPLIANCE")]
        WaitingForExportCompliance,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
    }

    impl ::std::fmt::Display for AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::DeveloperRejected => f.write_str("DEVELOPER_REJECTED"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::InvalidBinary => f.write_str("INVALID_BINARY"),
                Self::MetadataRejected => f.write_str("METADATA_REJECTED"),
                Self::PendingAppleRelease => f.write_str("PENDING_APPLE_RELEASE"),
                Self::PendingDeveloperRelease => f.write_str("PENDING_DEVELOPER_RELEASE"),
                Self::PrepareForSubmission => f.write_str("PREPARE_FOR_SUBMISSION"),
                Self::ProcessingForDistribution => f.write_str("PROCESSING_FOR_DISTRIBUTION"),
                Self::ReadyForDistribution => f.write_str("READY_FOR_DISTRIBUTION"),
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::ReplacedWithNewVersion => f.write_str("REPLACED_WITH_NEW_VERSION"),
                Self::WaitingForExportCompliance => f.write_str("WAITING_FOR_EXPORT_COMPLIANCE"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACCEPTED" => Ok(Self::Accepted),
                "DEVELOPER_REJECTED" => Ok(Self::DeveloperRejected),
                "IN_REVIEW" => Ok(Self::InReview),
                "INVALID_BINARY" => Ok(Self::InvalidBinary),
                "METADATA_REJECTED" => Ok(Self::MetadataRejected),
                "PENDING_APPLE_RELEASE" => Ok(Self::PendingAppleRelease),
                "PENDING_DEVELOPER_RELEASE" => Ok(Self::PendingDeveloperRelease),
                "PREPARE_FOR_SUBMISSION" => Ok(Self::PrepareForSubmission),
                "PROCESSING_FOR_DISTRIBUTION" => Ok(Self::ProcessingForDistribution),
                "READY_FOR_DISTRIBUTION" => Ok(Self::ReadyForDistribution),
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "REJECTED" => Ok(Self::Rejected),
                "REPLACED_WITH_NEW_VERSION" => Ok(Self::ReplacedWithNewVersion),
                "WAITING_FOR_EXPORT_COMPLIANCE" => Ok(Self::WaitingForExportCompliance),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFilterAppStoreVersionsPlatformItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "IOS",
    ///    "MAC_OS",
    ///    "TV_OS",
    ///    "VISION_OS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFilterAppStoreVersionsPlatformItem {
        #[serde(rename = "IOS")]
        Ios,
        #[serde(rename = "MAC_OS")]
        MacOs,
        #[serde(rename = "TV_OS")]
        TvOs,
        #[serde(rename = "VISION_OS")]
        VisionOs,
    }

    impl ::std::fmt::Display for AppsGetCollectionFilterAppStoreVersionsPlatformItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ios => f.write_str("IOS"),
                Self::MacOs => f.write_str("MAC_OS"),
                Self::TvOs => f.write_str("TV_OS"),
                Self::VisionOs => f.write_str("VISION_OS"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFilterAppStoreVersionsPlatformItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "IOS" => Ok(Self::Ios),
                "MAC_OS" => Ok(Self::MacOs),
                "TV_OS" => Ok(Self::TvOs),
                "VISION_OS" => Ok(Self::VisionOs),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFilterAppStoreVersionsPlatformItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFilterAppStoreVersionsPlatformItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFilterAppStoreVersionsPlatformItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFilterReviewSubmissionsPlatformItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "IOS",
    ///    "MAC_OS",
    ///    "TV_OS",
    ///    "VISION_OS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFilterReviewSubmissionsPlatformItem {
        #[serde(rename = "IOS")]
        Ios,
        #[serde(rename = "MAC_OS")]
        MacOs,
        #[serde(rename = "TV_OS")]
        TvOs,
        #[serde(rename = "VISION_OS")]
        VisionOs,
    }

    impl ::std::fmt::Display for AppsGetCollectionFilterReviewSubmissionsPlatformItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ios => f.write_str("IOS"),
                Self::MacOs => f.write_str("MAC_OS"),
                Self::TvOs => f.write_str("TV_OS"),
                Self::VisionOs => f.write_str("VISION_OS"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFilterReviewSubmissionsPlatformItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "IOS" => Ok(Self::Ios),
                "MAC_OS" => Ok(Self::MacOs),
                "TV_OS" => Ok(Self::TvOs),
                "VISION_OS" => Ok(Self::VisionOs),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFilterReviewSubmissionsPlatformItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFilterReviewSubmissionsPlatformItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFilterReviewSubmissionsPlatformItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFilterReviewSubmissionsStateItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "READY_FOR_REVIEW",
    ///    "WAITING_FOR_REVIEW",
    ///    "IN_REVIEW",
    ///    "UNRESOLVED_ISSUES",
    ///    "CANCELING",
    ///    "COMPLETING",
    ///    "COMPLETE"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFilterReviewSubmissionsStateItem {
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "UNRESOLVED_ISSUES")]
        UnresolvedIssues,
        #[serde(rename = "CANCELING")]
        Canceling,
        #[serde(rename = "COMPLETING")]
        Completing,
        #[serde(rename = "COMPLETE")]
        Complete,
    }

    impl ::std::fmt::Display for AppsGetCollectionFilterReviewSubmissionsStateItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::UnresolvedIssues => f.write_str("UNRESOLVED_ISSUES"),
                Self::Canceling => f.write_str("CANCELING"),
                Self::Completing => f.write_str("COMPLETING"),
                Self::Complete => f.write_str("COMPLETE"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFilterReviewSubmissionsStateItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                "IN_REVIEW" => Ok(Self::InReview),
                "UNRESOLVED_ISSUES" => Ok(Self::UnresolvedIssues),
                "CANCELING" => Ok(Self::Canceling),
                "COMPLETING" => Ok(Self::Completing),
                "COMPLETE" => Ok(Self::Complete),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFilterReviewSubmissionsStateItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFilterReviewSubmissionsStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFilterReviewSubmissionsStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionIncludeItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appEncryptionDeclarations",
    ///    "appStoreIcon",
    ///    "ciProduct",
    ///    "betaGroups",
    ///    "appStoreVersions",
    ///    "preReleaseVersions",
    ///    "betaAppLocalizations",
    ///    "builds",
    ///    "betaLicenseAgreement",
    ///    "betaAppReviewDetail",
    ///    "appInfos",
    ///    "appClips",
    ///    "endUserLicenseAgreement",
    ///    "inAppPurchases",
    ///    "subscriptionGroups",
    ///    "gameCenterEnabledVersions",
    ///    "appCustomProductPages",
    ///    "inAppPurchasesV2",
    ///    "promotedPurchases",
    ///    "appEvents",
    ///    "reviewSubmissions",
    ///    "subscriptionGracePeriod",
    ///    "gameCenterDetail",
    ///    "appStoreVersionExperimentsV2",
    ///    "androidToIosAppMappingDetails"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionIncludeItem {
        #[serde(rename = "appEncryptionDeclarations")]
        AppEncryptionDeclarations,
        #[serde(rename = "appStoreIcon")]
        AppStoreIcon,
        #[serde(rename = "ciProduct")]
        CiProduct,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
        #[serde(rename = "preReleaseVersions")]
        PreReleaseVersions,
        #[serde(rename = "betaAppLocalizations")]
        BetaAppLocalizations,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaLicenseAgreement")]
        BetaLicenseAgreement,
        #[serde(rename = "betaAppReviewDetail")]
        BetaAppReviewDetail,
        #[serde(rename = "appInfos")]
        AppInfos,
        #[serde(rename = "appClips")]
        AppClips,
        #[serde(rename = "endUserLicenseAgreement")]
        EndUserLicenseAgreement,
        #[serde(rename = "inAppPurchases")]
        InAppPurchases,
        #[serde(rename = "subscriptionGroups")]
        SubscriptionGroups,
        #[serde(rename = "gameCenterEnabledVersions")]
        GameCenterEnabledVersions,
        #[serde(rename = "appCustomProductPages")]
        AppCustomProductPages,
        #[serde(rename = "inAppPurchasesV2")]
        InAppPurchasesV2,
        #[serde(rename = "promotedPurchases")]
        PromotedPurchases,
        #[serde(rename = "appEvents")]
        AppEvents,
        #[serde(rename = "reviewSubmissions")]
        ReviewSubmissions,
        #[serde(rename = "subscriptionGracePeriod")]
        SubscriptionGracePeriod,
        #[serde(rename = "gameCenterDetail")]
        GameCenterDetail,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "androidToIosAppMappingDetails")]
        AndroidToIosAppMappingDetails,
    }

    impl ::std::fmt::Display for AppsGetCollectionIncludeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppEncryptionDeclarations => f.write_str("appEncryptionDeclarations"),
                Self::AppStoreIcon => f.write_str("appStoreIcon"),
                Self::CiProduct => f.write_str("ciProduct"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
                Self::PreReleaseVersions => f.write_str("preReleaseVersions"),
                Self::BetaAppLocalizations => f.write_str("betaAppLocalizations"),
                Self::Builds => f.write_str("builds"),
                Self::BetaLicenseAgreement => f.write_str("betaLicenseAgreement"),
                Self::BetaAppReviewDetail => f.write_str("betaAppReviewDetail"),
                Self::AppInfos => f.write_str("appInfos"),
                Self::AppClips => f.write_str("appClips"),
                Self::EndUserLicenseAgreement => f.write_str("endUserLicenseAgreement"),
                Self::InAppPurchases => f.write_str("inAppPurchases"),
                Self::SubscriptionGroups => f.write_str("subscriptionGroups"),
                Self::GameCenterEnabledVersions => f.write_str("gameCenterEnabledVersions"),
                Self::AppCustomProductPages => f.write_str("appCustomProductPages"),
                Self::InAppPurchasesV2 => f.write_str("inAppPurchasesV2"),
                Self::PromotedPurchases => f.write_str("promotedPurchases"),
                Self::AppEvents => f.write_str("appEvents"),
                Self::ReviewSubmissions => f.write_str("reviewSubmissions"),
                Self::SubscriptionGracePeriod => f.write_str("subscriptionGracePeriod"),
                Self::GameCenterDetail => f.write_str("gameCenterDetail"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AndroidToIosAppMappingDetails => f.write_str("androidToIosAppMappingDetails"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionIncludeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appEncryptionDeclarations" => Ok(Self::AppEncryptionDeclarations),
                "appStoreIcon" => Ok(Self::AppStoreIcon),
                "ciProduct" => Ok(Self::CiProduct),
                "betaGroups" => Ok(Self::BetaGroups),
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                "preReleaseVersions" => Ok(Self::PreReleaseVersions),
                "betaAppLocalizations" => Ok(Self::BetaAppLocalizations),
                "builds" => Ok(Self::Builds),
                "betaLicenseAgreement" => Ok(Self::BetaLicenseAgreement),
                "betaAppReviewDetail" => Ok(Self::BetaAppReviewDetail),
                "appInfos" => Ok(Self::AppInfos),
                "appClips" => Ok(Self::AppClips),
                "endUserLicenseAgreement" => Ok(Self::EndUserLicenseAgreement),
                "inAppPurchases" => Ok(Self::InAppPurchases),
                "subscriptionGroups" => Ok(Self::SubscriptionGroups),
                "gameCenterEnabledVersions" => Ok(Self::GameCenterEnabledVersions),
                "appCustomProductPages" => Ok(Self::AppCustomProductPages),
                "inAppPurchasesV2" => Ok(Self::InAppPurchasesV2),
                "promotedPurchases" => Ok(Self::PromotedPurchases),
                "appEvents" => Ok(Self::AppEvents),
                "reviewSubmissions" => Ok(Self::ReviewSubmissions),
                "subscriptionGracePeriod" => Ok(Self::SubscriptionGracePeriod),
                "gameCenterDetail" => Ok(Self::GameCenterDetail),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "androidToIosAppMappingDetails" => Ok(Self::AndroidToIosAppMappingDetails),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionSortItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "-name",
    ///    "bundleId",
    ///    "-bundleId",
    ///    "sku",
    ///    "-sku"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionSortItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "-name")]
        Xname,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "-bundleId")]
        XbundleId,
        #[serde(rename = "sku")]
        Sku,
        #[serde(rename = "-sku")]
        Xsku,
    }

    impl ::std::fmt::Display for AppsGetCollectionSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::Xname => f.write_str("-name"),
                Self::BundleId => f.write_str("bundleId"),
                Self::XbundleId => f.write_str("-bundleId"),
                Self::Sku => f.write_str("sku"),
                Self::Xsku => f.write_str("-sku"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "-name" => Ok(Self::Xname),
                "bundleId" => Ok(Self::BundleId),
                "-bundleId" => Ok(Self::XbundleId),
                "sku" => Ok(Self::Sku),
                "-sku" => Ok(Self::Xsku),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "packageName",
    ///    "appSigningKeyPublicCertificateSha256Fingerprints"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem {
        #[serde(rename = "packageName")]
        PackageName,
        #[serde(rename = "appSigningKeyPublicCertificateSha256Fingerprints")]
        AppSigningKeyPublicCertificateSha256Fingerprints,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::PackageName => f.write_str("packageName"),
                Self::AppSigningKeyPublicCertificateSha256Fingerprints => {
                    f.write_str("appSigningKeyPublicCertificateSha256Fingerprints")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "packageName" => Ok(Self::PackageName),
                "appSigningKeyPublicCertificateSha256Fingerprints" => {
                    Ok(Self::AppSigningKeyPublicCertificateSha256Fingerprints)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppClipsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "bundleId",
    ///    "app",
    ///    "appClipDefaultExperiences",
    ///    "appClipAdvancedExperiences"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppClipsItem {
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appClipDefaultExperiences")]
        AppClipDefaultExperiences,
        #[serde(rename = "appClipAdvancedExperiences")]
        AppClipAdvancedExperiences,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppClipsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::BundleId => f.write_str("bundleId"),
                Self::App => f.write_str("app"),
                Self::AppClipDefaultExperiences => f.write_str("appClipDefaultExperiences"),
                Self::AppClipAdvancedExperiences => f.write_str("appClipAdvancedExperiences"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppClipsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "bundleId" => Ok(Self::BundleId),
                "app" => Ok(Self::App),
                "appClipDefaultExperiences" => Ok(Self::AppClipDefaultExperiences),
                "appClipAdvancedExperiences" => Ok(Self::AppClipAdvancedExperiences),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppClipsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsAppClipsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsAppClipsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppCustomProductPagesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "url",
    ///    "visible",
    ///    "app",
    ///    "appCustomProductPageVersions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppCustomProductPagesItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "url")]
        Url,
        #[serde(rename = "visible")]
        Visible,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appCustomProductPageVersions")]
        AppCustomProductPageVersions,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppCustomProductPagesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::Url => f.write_str("url"),
                Self::Visible => f.write_str("visible"),
                Self::App => f.write_str("app"),
                Self::AppCustomProductPageVersions => f.write_str("appCustomProductPageVersions"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppCustomProductPagesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "url" => Ok(Self::Url),
                "visible" => Ok(Self::Visible),
                "app" => Ok(Self::App),
                "appCustomProductPageVersions" => Ok(Self::AppCustomProductPageVersions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppCustomProductPagesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsAppCustomProductPagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsAppCustomProductPagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppEncryptionDeclarationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appDescription",
    ///    "createdDate",
    ///    "usesEncryption",
    ///    "exempt",
    ///    "containsProprietaryCryptography",
    ///    "containsThirdPartyCryptography",
    ///    "availableOnFrenchStore",
    ///    "platform",
    ///    "uploadedDate",
    ///    "documentUrl",
    ///    "documentName",
    ///    "documentType",
    ///    "appEncryptionDeclarationState",
    ///    "codeValue",
    ///    "app",
    ///    "builds",
    ///    "appEncryptionDeclarationDocument"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppEncryptionDeclarationsItem {
        #[serde(rename = "appDescription")]
        AppDescription,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "usesEncryption")]
        UsesEncryption,
        #[serde(rename = "exempt")]
        Exempt,
        #[serde(rename = "containsProprietaryCryptography")]
        ContainsProprietaryCryptography,
        #[serde(rename = "containsThirdPartyCryptography")]
        ContainsThirdPartyCryptography,
        #[serde(rename = "availableOnFrenchStore")]
        AvailableOnFrenchStore,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "uploadedDate")]
        UploadedDate,
        #[serde(rename = "documentUrl")]
        DocumentUrl,
        #[serde(rename = "documentName")]
        DocumentName,
        #[serde(rename = "documentType")]
        DocumentType,
        #[serde(rename = "appEncryptionDeclarationState")]
        AppEncryptionDeclarationState,
        #[serde(rename = "codeValue")]
        CodeValue,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "appEncryptionDeclarationDocument")]
        AppEncryptionDeclarationDocument,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppEncryptionDeclarationsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppDescription => f.write_str("appDescription"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::UsesEncryption => f.write_str("usesEncryption"),
                Self::Exempt => f.write_str("exempt"),
                Self::ContainsProprietaryCryptography => {
                    f.write_str("containsProprietaryCryptography")
                }
                Self::ContainsThirdPartyCryptography => {
                    f.write_str("containsThirdPartyCryptography")
                }
                Self::AvailableOnFrenchStore => f.write_str("availableOnFrenchStore"),
                Self::Platform => f.write_str("platform"),
                Self::UploadedDate => f.write_str("uploadedDate"),
                Self::DocumentUrl => f.write_str("documentUrl"),
                Self::DocumentName => f.write_str("documentName"),
                Self::DocumentType => f.write_str("documentType"),
                Self::AppEncryptionDeclarationState => f.write_str("appEncryptionDeclarationState"),
                Self::CodeValue => f.write_str("codeValue"),
                Self::App => f.write_str("app"),
                Self::Builds => f.write_str("builds"),
                Self::AppEncryptionDeclarationDocument => {
                    f.write_str("appEncryptionDeclarationDocument")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppEncryptionDeclarationsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appDescription" => Ok(Self::AppDescription),
                "createdDate" => Ok(Self::CreatedDate),
                "usesEncryption" => Ok(Self::UsesEncryption),
                "exempt" => Ok(Self::Exempt),
                "containsProprietaryCryptography" => Ok(Self::ContainsProprietaryCryptography),
                "containsThirdPartyCryptography" => Ok(Self::ContainsThirdPartyCryptography),
                "availableOnFrenchStore" => Ok(Self::AvailableOnFrenchStore),
                "platform" => Ok(Self::Platform),
                "uploadedDate" => Ok(Self::UploadedDate),
                "documentUrl" => Ok(Self::DocumentUrl),
                "documentName" => Ok(Self::DocumentName),
                "documentType" => Ok(Self::DocumentType),
                "appEncryptionDeclarationState" => Ok(Self::AppEncryptionDeclarationState),
                "codeValue" => Ok(Self::CodeValue),
                "app" => Ok(Self::App),
                "builds" => Ok(Self::Builds),
                "appEncryptionDeclarationDocument" => Ok(Self::AppEncryptionDeclarationDocument),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppEncryptionDeclarationsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsAppEncryptionDeclarationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsAppEncryptionDeclarationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppEventsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "referenceName",
    ///    "badge",
    ///    "eventState",
    ///    "deepLink",
    ///    "purchaseRequirement",
    ///    "primaryLocale",
    ///    "priority",
    ///    "purpose",
    ///    "territorySchedules",
    ///    "archivedTerritorySchedules",
    ///    "localizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppEventsItem {
        #[serde(rename = "referenceName")]
        ReferenceName,
        #[serde(rename = "badge")]
        Badge,
        #[serde(rename = "eventState")]
        EventState,
        #[serde(rename = "deepLink")]
        DeepLink,
        #[serde(rename = "purchaseRequirement")]
        PurchaseRequirement,
        #[serde(rename = "primaryLocale")]
        PrimaryLocale,
        #[serde(rename = "priority")]
        Priority,
        #[serde(rename = "purpose")]
        Purpose,
        #[serde(rename = "territorySchedules")]
        TerritorySchedules,
        #[serde(rename = "archivedTerritorySchedules")]
        ArchivedTerritorySchedules,
        #[serde(rename = "localizations")]
        Localizations,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppEventsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReferenceName => f.write_str("referenceName"),
                Self::Badge => f.write_str("badge"),
                Self::EventState => f.write_str("eventState"),
                Self::DeepLink => f.write_str("deepLink"),
                Self::PurchaseRequirement => f.write_str("purchaseRequirement"),
                Self::PrimaryLocale => f.write_str("primaryLocale"),
                Self::Priority => f.write_str("priority"),
                Self::Purpose => f.write_str("purpose"),
                Self::TerritorySchedules => f.write_str("territorySchedules"),
                Self::ArchivedTerritorySchedules => f.write_str("archivedTerritorySchedules"),
                Self::Localizations => f.write_str("localizations"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppEventsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "referenceName" => Ok(Self::ReferenceName),
                "badge" => Ok(Self::Badge),
                "eventState" => Ok(Self::EventState),
                "deepLink" => Ok(Self::DeepLink),
                "purchaseRequirement" => Ok(Self::PurchaseRequirement),
                "primaryLocale" => Ok(Self::PrimaryLocale),
                "priority" => Ok(Self::Priority),
                "purpose" => Ok(Self::Purpose),
                "territorySchedules" => Ok(Self::TerritorySchedules),
                "archivedTerritorySchedules" => Ok(Self::ArchivedTerritorySchedules),
                "localizations" => Ok(Self::Localizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsAppEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsAppEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppInfosItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreState",
    ///    "state",
    ///    "appStoreAgeRating",
    ///    "australiaAgeRating",
    ///    "brazilAgeRating",
    ///    "brazilAgeRatingV2",
    ///    "franceAgeRating",
    ///    "koreaAgeRating",
    ///    "app",
    ///    "ageRatingDeclaration",
    ///    "appInfoLocalizations",
    ///    "primaryCategory",
    ///    "primarySubcategoryOne",
    ///    "primarySubcategoryTwo",
    ///    "secondaryCategory",
    ///    "secondarySubcategoryOne",
    ///    "secondarySubcategoryTwo",
    ///    "territoryAgeRatings"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppInfosItem {
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "appStoreAgeRating")]
        AppStoreAgeRating,
        #[serde(rename = "australiaAgeRating")]
        AustraliaAgeRating,
        #[serde(rename = "brazilAgeRating")]
        BrazilAgeRating,
        #[serde(rename = "brazilAgeRatingV2")]
        BrazilAgeRatingV2,
        #[serde(rename = "franceAgeRating")]
        FranceAgeRating,
        #[serde(rename = "koreaAgeRating")]
        KoreaAgeRating,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "ageRatingDeclaration")]
        AgeRatingDeclaration,
        #[serde(rename = "appInfoLocalizations")]
        AppInfoLocalizations,
        #[serde(rename = "primaryCategory")]
        PrimaryCategory,
        #[serde(rename = "primarySubcategoryOne")]
        PrimarySubcategoryOne,
        #[serde(rename = "primarySubcategoryTwo")]
        PrimarySubcategoryTwo,
        #[serde(rename = "secondaryCategory")]
        SecondaryCategory,
        #[serde(rename = "secondarySubcategoryOne")]
        SecondarySubcategoryOne,
        #[serde(rename = "secondarySubcategoryTwo")]
        SecondarySubcategoryTwo,
        #[serde(rename = "territoryAgeRatings")]
        TerritoryAgeRatings,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppInfosItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::State => f.write_str("state"),
                Self::AppStoreAgeRating => f.write_str("appStoreAgeRating"),
                Self::AustraliaAgeRating => f.write_str("australiaAgeRating"),
                Self::BrazilAgeRating => f.write_str("brazilAgeRating"),
                Self::BrazilAgeRatingV2 => f.write_str("brazilAgeRatingV2"),
                Self::FranceAgeRating => f.write_str("franceAgeRating"),
                Self::KoreaAgeRating => f.write_str("koreaAgeRating"),
                Self::App => f.write_str("app"),
                Self::AgeRatingDeclaration => f.write_str("ageRatingDeclaration"),
                Self::AppInfoLocalizations => f.write_str("appInfoLocalizations"),
                Self::PrimaryCategory => f.write_str("primaryCategory"),
                Self::PrimarySubcategoryOne => f.write_str("primarySubcategoryOne"),
                Self::PrimarySubcategoryTwo => f.write_str("primarySubcategoryTwo"),
                Self::SecondaryCategory => f.write_str("secondaryCategory"),
                Self::SecondarySubcategoryOne => f.write_str("secondarySubcategoryOne"),
                Self::SecondarySubcategoryTwo => f.write_str("secondarySubcategoryTwo"),
                Self::TerritoryAgeRatings => f.write_str("territoryAgeRatings"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppInfosItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreState" => Ok(Self::AppStoreState),
                "state" => Ok(Self::State),
                "appStoreAgeRating" => Ok(Self::AppStoreAgeRating),
                "australiaAgeRating" => Ok(Self::AustraliaAgeRating),
                "brazilAgeRating" => Ok(Self::BrazilAgeRating),
                "brazilAgeRatingV2" => Ok(Self::BrazilAgeRatingV2),
                "franceAgeRating" => Ok(Self::FranceAgeRating),
                "koreaAgeRating" => Ok(Self::KoreaAgeRating),
                "app" => Ok(Self::App),
                "ageRatingDeclaration" => Ok(Self::AgeRatingDeclaration),
                "appInfoLocalizations" => Ok(Self::AppInfoLocalizations),
                "primaryCategory" => Ok(Self::PrimaryCategory),
                "primarySubcategoryOne" => Ok(Self::PrimarySubcategoryOne),
                "primarySubcategoryTwo" => Ok(Self::PrimarySubcategoryTwo),
                "secondaryCategory" => Ok(Self::SecondaryCategory),
                "secondarySubcategoryOne" => Ok(Self::SecondarySubcategoryOne),
                "secondarySubcategoryTwo" => Ok(Self::SecondarySubcategoryTwo),
                "territoryAgeRatings" => Ok(Self::TerritoryAgeRatings),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppStoreVersionExperimentsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "platform",
    ///    "trafficProportion",
    ///    "state",
    ///    "reviewRequired",
    ///    "startDate",
    ///    "endDate",
    ///    "app",
    ///    "latestControlVersion",
    ///    "controlVersions",
    ///    "appStoreVersionExperimentTreatments"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppStoreVersionExperimentsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "trafficProportion")]
        TrafficProportion,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "reviewRequired")]
        ReviewRequired,
        #[serde(rename = "startDate")]
        StartDate,
        #[serde(rename = "endDate")]
        EndDate,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "latestControlVersion")]
        LatestControlVersion,
        #[serde(rename = "controlVersions")]
        ControlVersions,
        #[serde(rename = "appStoreVersionExperimentTreatments")]
        AppStoreVersionExperimentTreatments,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppStoreVersionExperimentsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::Platform => f.write_str("platform"),
                Self::TrafficProportion => f.write_str("trafficProportion"),
                Self::State => f.write_str("state"),
                Self::ReviewRequired => f.write_str("reviewRequired"),
                Self::StartDate => f.write_str("startDate"),
                Self::EndDate => f.write_str("endDate"),
                Self::App => f.write_str("app"),
                Self::LatestControlVersion => f.write_str("latestControlVersion"),
                Self::ControlVersions => f.write_str("controlVersions"),
                Self::AppStoreVersionExperimentTreatments => {
                    f.write_str("appStoreVersionExperimentTreatments")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppStoreVersionExperimentsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "platform" => Ok(Self::Platform),
                "trafficProportion" => Ok(Self::TrafficProportion),
                "state" => Ok(Self::State),
                "reviewRequired" => Ok(Self::ReviewRequired),
                "startDate" => Ok(Self::StartDate),
                "endDate" => Ok(Self::EndDate),
                "app" => Ok(Self::App),
                "latestControlVersion" => Ok(Self::LatestControlVersion),
                "controlVersions" => Ok(Self::ControlVersions),
                "appStoreVersionExperimentTreatments" => {
                    Ok(Self::AppStoreVersionExperimentTreatments)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppStoreVersionExperimentsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppStoreVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "versionString",
    ///    "appStoreState",
    ///    "appVersionState",
    ///    "copyright",
    ///    "reviewType",
    ///    "releaseType",
    ///    "earliestReleaseDate",
    ///    "usesIdfa",
    ///    "downloadable",
    ///    "createdDate",
    ///    "app",
    ///    "appStoreVersionLocalizations",
    ///    "build",
    ///    "appStoreVersionPhasedRelease",
    ///    "gameCenterAppVersion",
    ///    "routingAppCoverage",
    ///    "appStoreReviewDetail",
    ///    "appStoreVersionSubmission",
    ///    "appClipDefaultExperience",
    ///    "appStoreVersionExperiments",
    ///    "appStoreVersionExperimentsV2",
    ///    "customerReviews",
    ///    "alternativeDistributionPackage"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppStoreVersionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "versionString")]
        VersionString,
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "appVersionState")]
        AppVersionState,
        #[serde(rename = "copyright")]
        Copyright,
        #[serde(rename = "reviewType")]
        ReviewType,
        #[serde(rename = "releaseType")]
        ReleaseType,
        #[serde(rename = "earliestReleaseDate")]
        EarliestReleaseDate,
        #[serde(rename = "usesIdfa")]
        UsesIdfa,
        #[serde(rename = "downloadable")]
        Downloadable,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "appStoreVersionPhasedRelease")]
        AppStoreVersionPhasedRelease,
        #[serde(rename = "gameCenterAppVersion")]
        GameCenterAppVersion,
        #[serde(rename = "routingAppCoverage")]
        RoutingAppCoverage,
        #[serde(rename = "appStoreReviewDetail")]
        AppStoreReviewDetail,
        #[serde(rename = "appStoreVersionSubmission")]
        AppStoreVersionSubmission,
        #[serde(rename = "appClipDefaultExperience")]
        AppClipDefaultExperience,
        #[serde(rename = "appStoreVersionExperiments")]
        AppStoreVersionExperiments,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "alternativeDistributionPackage")]
        AlternativeDistributionPackage,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppStoreVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::VersionString => f.write_str("versionString"),
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::AppVersionState => f.write_str("appVersionState"),
                Self::Copyright => f.write_str("copyright"),
                Self::ReviewType => f.write_str("reviewType"),
                Self::ReleaseType => f.write_str("releaseType"),
                Self::EarliestReleaseDate => f.write_str("earliestReleaseDate"),
                Self::UsesIdfa => f.write_str("usesIdfa"),
                Self::Downloadable => f.write_str("downloadable"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::App => f.write_str("app"),
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
                Self::Build => f.write_str("build"),
                Self::AppStoreVersionPhasedRelease => f.write_str("appStoreVersionPhasedRelease"),
                Self::GameCenterAppVersion => f.write_str("gameCenterAppVersion"),
                Self::RoutingAppCoverage => f.write_str("routingAppCoverage"),
                Self::AppStoreReviewDetail => f.write_str("appStoreReviewDetail"),
                Self::AppStoreVersionSubmission => f.write_str("appStoreVersionSubmission"),
                Self::AppClipDefaultExperience => f.write_str("appClipDefaultExperience"),
                Self::AppStoreVersionExperiments => f.write_str("appStoreVersionExperiments"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::AlternativeDistributionPackage => {
                    f.write_str("alternativeDistributionPackage")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppStoreVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "versionString" => Ok(Self::VersionString),
                "appStoreState" => Ok(Self::AppStoreState),
                "appVersionState" => Ok(Self::AppVersionState),
                "copyright" => Ok(Self::Copyright),
                "reviewType" => Ok(Self::ReviewType),
                "releaseType" => Ok(Self::ReleaseType),
                "earliestReleaseDate" => Ok(Self::EarliestReleaseDate),
                "usesIdfa" => Ok(Self::UsesIdfa),
                "downloadable" => Ok(Self::Downloadable),
                "createdDate" => Ok(Self::CreatedDate),
                "app" => Ok(Self::App),
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                "build" => Ok(Self::Build),
                "appStoreVersionPhasedRelease" => Ok(Self::AppStoreVersionPhasedRelease),
                "gameCenterAppVersion" => Ok(Self::GameCenterAppVersion),
                "routingAppCoverage" => Ok(Self::RoutingAppCoverage),
                "appStoreReviewDetail" => Ok(Self::AppStoreReviewDetail),
                "appStoreVersionSubmission" => Ok(Self::AppStoreVersionSubmission),
                "appClipDefaultExperience" => Ok(Self::AppClipDefaultExperience),
                "appStoreVersionExperiments" => Ok(Self::AppStoreVersionExperiments),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "customerReviews" => Ok(Self::CustomerReviews),
                "alternativeDistributionPackage" => Ok(Self::AlternativeDistributionPackage),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppStoreVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsAppStoreVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsAppStoreVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "accessibilityUrl",
    ///    "name",
    ///    "bundleId",
    ///    "sku",
    ///    "primaryLocale",
    ///    "isOrEverWasMadeForKids",
    ///    "subscriptionStatusUrl",
    ///    "subscriptionStatusUrlVersion",
    ///    "subscriptionStatusUrlForSandbox",
    ///    "subscriptionStatusUrlVersionForSandbox",
    ///    "contentRightsDeclaration",
    ///    "streamlinedPurchasingEnabled",
    ///    "accessibilityDeclarations",
    ///    "appEncryptionDeclarations",
    ///    "appStoreIcon",
    ///    "ciProduct",
    ///    "betaTesters",
    ///    "betaGroups",
    ///    "appStoreVersions",
    ///    "appTags",
    ///    "preReleaseVersions",
    ///    "betaAppLocalizations",
    ///    "builds",
    ///    "betaLicenseAgreement",
    ///    "betaAppReviewDetail",
    ///    "appInfos",
    ///    "appClips",
    ///    "appPricePoints",
    ///    "endUserLicenseAgreement",
    ///    "appPriceSchedule",
    ///    "appAvailabilityV2",
    ///    "inAppPurchases",
    ///    "subscriptionGroups",
    ///    "gameCenterEnabledVersions",
    ///    "perfPowerMetrics",
    ///    "appCustomProductPages",
    ///    "inAppPurchasesV2",
    ///    "promotedPurchases",
    ///    "appEvents",
    ///    "reviewSubmissions",
    ///    "subscriptionGracePeriod",
    ///    "customerReviews",
    ///    "customerReviewSummarizations",
    ///    "gameCenterDetail",
    ///    "appStoreVersionExperimentsV2",
    ///    "alternativeDistributionKey",
    ///    "analyticsReportRequests",
    ///    "marketplaceSearchDetail",
    ///    "buildUploads",
    ///    "backgroundAssets",
    ///    "betaFeedbackScreenshotSubmissions",
    ///    "betaFeedbackCrashSubmissions",
    ///    "searchKeywords",
    ///    "webhooks",
    ///    "androidToIosAppMappingDetails"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppsItem {
        #[serde(rename = "accessibilityUrl")]
        AccessibilityUrl,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "sku")]
        Sku,
        #[serde(rename = "primaryLocale")]
        PrimaryLocale,
        #[serde(rename = "isOrEverWasMadeForKids")]
        IsOrEverWasMadeForKids,
        #[serde(rename = "subscriptionStatusUrl")]
        SubscriptionStatusUrl,
        #[serde(rename = "subscriptionStatusUrlVersion")]
        SubscriptionStatusUrlVersion,
        #[serde(rename = "subscriptionStatusUrlForSandbox")]
        SubscriptionStatusUrlForSandbox,
        #[serde(rename = "subscriptionStatusUrlVersionForSandbox")]
        SubscriptionStatusUrlVersionForSandbox,
        #[serde(rename = "contentRightsDeclaration")]
        ContentRightsDeclaration,
        #[serde(rename = "streamlinedPurchasingEnabled")]
        StreamlinedPurchasingEnabled,
        #[serde(rename = "accessibilityDeclarations")]
        AccessibilityDeclarations,
        #[serde(rename = "appEncryptionDeclarations")]
        AppEncryptionDeclarations,
        #[serde(rename = "appStoreIcon")]
        AppStoreIcon,
        #[serde(rename = "ciProduct")]
        CiProduct,
        #[serde(rename = "betaTesters")]
        BetaTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
        #[serde(rename = "appTags")]
        AppTags,
        #[serde(rename = "preReleaseVersions")]
        PreReleaseVersions,
        #[serde(rename = "betaAppLocalizations")]
        BetaAppLocalizations,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaLicenseAgreement")]
        BetaLicenseAgreement,
        #[serde(rename = "betaAppReviewDetail")]
        BetaAppReviewDetail,
        #[serde(rename = "appInfos")]
        AppInfos,
        #[serde(rename = "appClips")]
        AppClips,
        #[serde(rename = "appPricePoints")]
        AppPricePoints,
        #[serde(rename = "endUserLicenseAgreement")]
        EndUserLicenseAgreement,
        #[serde(rename = "appPriceSchedule")]
        AppPriceSchedule,
        #[serde(rename = "appAvailabilityV2")]
        AppAvailabilityV2,
        #[serde(rename = "inAppPurchases")]
        InAppPurchases,
        #[serde(rename = "subscriptionGroups")]
        SubscriptionGroups,
        #[serde(rename = "gameCenterEnabledVersions")]
        GameCenterEnabledVersions,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "appCustomProductPages")]
        AppCustomProductPages,
        #[serde(rename = "inAppPurchasesV2")]
        InAppPurchasesV2,
        #[serde(rename = "promotedPurchases")]
        PromotedPurchases,
        #[serde(rename = "appEvents")]
        AppEvents,
        #[serde(rename = "reviewSubmissions")]
        ReviewSubmissions,
        #[serde(rename = "subscriptionGracePeriod")]
        SubscriptionGracePeriod,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "customerReviewSummarizations")]
        CustomerReviewSummarizations,
        #[serde(rename = "gameCenterDetail")]
        GameCenterDetail,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "alternativeDistributionKey")]
        AlternativeDistributionKey,
        #[serde(rename = "analyticsReportRequests")]
        AnalyticsReportRequests,
        #[serde(rename = "marketplaceSearchDetail")]
        MarketplaceSearchDetail,
        #[serde(rename = "buildUploads")]
        BuildUploads,
        #[serde(rename = "backgroundAssets")]
        BackgroundAssets,
        #[serde(rename = "betaFeedbackScreenshotSubmissions")]
        BetaFeedbackScreenshotSubmissions,
        #[serde(rename = "betaFeedbackCrashSubmissions")]
        BetaFeedbackCrashSubmissions,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
        #[serde(rename = "webhooks")]
        Webhooks,
        #[serde(rename = "androidToIosAppMappingDetails")]
        AndroidToIosAppMappingDetails,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AccessibilityUrl => f.write_str("accessibilityUrl"),
                Self::Name => f.write_str("name"),
                Self::BundleId => f.write_str("bundleId"),
                Self::Sku => f.write_str("sku"),
                Self::PrimaryLocale => f.write_str("primaryLocale"),
                Self::IsOrEverWasMadeForKids => f.write_str("isOrEverWasMadeForKids"),
                Self::SubscriptionStatusUrl => f.write_str("subscriptionStatusUrl"),
                Self::SubscriptionStatusUrlVersion => f.write_str("subscriptionStatusUrlVersion"),
                Self::SubscriptionStatusUrlForSandbox => {
                    f.write_str("subscriptionStatusUrlForSandbox")
                }
                Self::SubscriptionStatusUrlVersionForSandbox => {
                    f.write_str("subscriptionStatusUrlVersionForSandbox")
                }
                Self::ContentRightsDeclaration => f.write_str("contentRightsDeclaration"),
                Self::StreamlinedPurchasingEnabled => f.write_str("streamlinedPurchasingEnabled"),
                Self::AccessibilityDeclarations => f.write_str("accessibilityDeclarations"),
                Self::AppEncryptionDeclarations => f.write_str("appEncryptionDeclarations"),
                Self::AppStoreIcon => f.write_str("appStoreIcon"),
                Self::CiProduct => f.write_str("ciProduct"),
                Self::BetaTesters => f.write_str("betaTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
                Self::AppTags => f.write_str("appTags"),
                Self::PreReleaseVersions => f.write_str("preReleaseVersions"),
                Self::BetaAppLocalizations => f.write_str("betaAppLocalizations"),
                Self::Builds => f.write_str("builds"),
                Self::BetaLicenseAgreement => f.write_str("betaLicenseAgreement"),
                Self::BetaAppReviewDetail => f.write_str("betaAppReviewDetail"),
                Self::AppInfos => f.write_str("appInfos"),
                Self::AppClips => f.write_str("appClips"),
                Self::AppPricePoints => f.write_str("appPricePoints"),
                Self::EndUserLicenseAgreement => f.write_str("endUserLicenseAgreement"),
                Self::AppPriceSchedule => f.write_str("appPriceSchedule"),
                Self::AppAvailabilityV2 => f.write_str("appAvailabilityV2"),
                Self::InAppPurchases => f.write_str("inAppPurchases"),
                Self::SubscriptionGroups => f.write_str("subscriptionGroups"),
                Self::GameCenterEnabledVersions => f.write_str("gameCenterEnabledVersions"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::AppCustomProductPages => f.write_str("appCustomProductPages"),
                Self::InAppPurchasesV2 => f.write_str("inAppPurchasesV2"),
                Self::PromotedPurchases => f.write_str("promotedPurchases"),
                Self::AppEvents => f.write_str("appEvents"),
                Self::ReviewSubmissions => f.write_str("reviewSubmissions"),
                Self::SubscriptionGracePeriod => f.write_str("subscriptionGracePeriod"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::CustomerReviewSummarizations => f.write_str("customerReviewSummarizations"),
                Self::GameCenterDetail => f.write_str("gameCenterDetail"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AlternativeDistributionKey => f.write_str("alternativeDistributionKey"),
                Self::AnalyticsReportRequests => f.write_str("analyticsReportRequests"),
                Self::MarketplaceSearchDetail => f.write_str("marketplaceSearchDetail"),
                Self::BuildUploads => f.write_str("buildUploads"),
                Self::BackgroundAssets => f.write_str("backgroundAssets"),
                Self::BetaFeedbackScreenshotSubmissions => {
                    f.write_str("betaFeedbackScreenshotSubmissions")
                }
                Self::BetaFeedbackCrashSubmissions => f.write_str("betaFeedbackCrashSubmissions"),
                Self::SearchKeywords => f.write_str("searchKeywords"),
                Self::Webhooks => f.write_str("webhooks"),
                Self::AndroidToIosAppMappingDetails => f.write_str("androidToIosAppMappingDetails"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "accessibilityUrl" => Ok(Self::AccessibilityUrl),
                "name" => Ok(Self::Name),
                "bundleId" => Ok(Self::BundleId),
                "sku" => Ok(Self::Sku),
                "primaryLocale" => Ok(Self::PrimaryLocale),
                "isOrEverWasMadeForKids" => Ok(Self::IsOrEverWasMadeForKids),
                "subscriptionStatusUrl" => Ok(Self::SubscriptionStatusUrl),
                "subscriptionStatusUrlVersion" => Ok(Self::SubscriptionStatusUrlVersion),
                "subscriptionStatusUrlForSandbox" => Ok(Self::SubscriptionStatusUrlForSandbox),
                "subscriptionStatusUrlVersionForSandbox" => {
                    Ok(Self::SubscriptionStatusUrlVersionForSandbox)
                }
                "contentRightsDeclaration" => Ok(Self::ContentRightsDeclaration),
                "streamlinedPurchasingEnabled" => Ok(Self::StreamlinedPurchasingEnabled),
                "accessibilityDeclarations" => Ok(Self::AccessibilityDeclarations),
                "appEncryptionDeclarations" => Ok(Self::AppEncryptionDeclarations),
                "appStoreIcon" => Ok(Self::AppStoreIcon),
                "ciProduct" => Ok(Self::CiProduct),
                "betaTesters" => Ok(Self::BetaTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                "appTags" => Ok(Self::AppTags),
                "preReleaseVersions" => Ok(Self::PreReleaseVersions),
                "betaAppLocalizations" => Ok(Self::BetaAppLocalizations),
                "builds" => Ok(Self::Builds),
                "betaLicenseAgreement" => Ok(Self::BetaLicenseAgreement),
                "betaAppReviewDetail" => Ok(Self::BetaAppReviewDetail),
                "appInfos" => Ok(Self::AppInfos),
                "appClips" => Ok(Self::AppClips),
                "appPricePoints" => Ok(Self::AppPricePoints),
                "endUserLicenseAgreement" => Ok(Self::EndUserLicenseAgreement),
                "appPriceSchedule" => Ok(Self::AppPriceSchedule),
                "appAvailabilityV2" => Ok(Self::AppAvailabilityV2),
                "inAppPurchases" => Ok(Self::InAppPurchases),
                "subscriptionGroups" => Ok(Self::SubscriptionGroups),
                "gameCenterEnabledVersions" => Ok(Self::GameCenterEnabledVersions),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "appCustomProductPages" => Ok(Self::AppCustomProductPages),
                "inAppPurchasesV2" => Ok(Self::InAppPurchasesV2),
                "promotedPurchases" => Ok(Self::PromotedPurchases),
                "appEvents" => Ok(Self::AppEvents),
                "reviewSubmissions" => Ok(Self::ReviewSubmissions),
                "subscriptionGracePeriod" => Ok(Self::SubscriptionGracePeriod),
                "customerReviews" => Ok(Self::CustomerReviews),
                "customerReviewSummarizations" => Ok(Self::CustomerReviewSummarizations),
                "gameCenterDetail" => Ok(Self::GameCenterDetail),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "alternativeDistributionKey" => Ok(Self::AlternativeDistributionKey),
                "analyticsReportRequests" => Ok(Self::AnalyticsReportRequests),
                "marketplaceSearchDetail" => Ok(Self::MarketplaceSearchDetail),
                "buildUploads" => Ok(Self::BuildUploads),
                "backgroundAssets" => Ok(Self::BackgroundAssets),
                "betaFeedbackScreenshotSubmissions" => Ok(Self::BetaFeedbackScreenshotSubmissions),
                "betaFeedbackCrashSubmissions" => Ok(Self::BetaFeedbackCrashSubmissions),
                "searchKeywords" => Ok(Self::SearchKeywords),
                "webhooks" => Ok(Self::Webhooks),
                "androidToIosAppMappingDetails" => Ok(Self::AndroidToIosAppMappingDetails),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsBetaAppLocalizationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "feedbackEmail",
    ///    "marketingUrl",
    ///    "privacyPolicyUrl",
    ///    "tvOsPrivacyPolicy",
    ///    "description",
    ///    "locale",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsBetaAppLocalizationsItem {
        #[serde(rename = "feedbackEmail")]
        FeedbackEmail,
        #[serde(rename = "marketingUrl")]
        MarketingUrl,
        #[serde(rename = "privacyPolicyUrl")]
        PrivacyPolicyUrl,
        #[serde(rename = "tvOsPrivacyPolicy")]
        TvOsPrivacyPolicy,
        #[serde(rename = "description")]
        Description,
        #[serde(rename = "locale")]
        Locale,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsBetaAppLocalizationsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::FeedbackEmail => f.write_str("feedbackEmail"),
                Self::MarketingUrl => f.write_str("marketingUrl"),
                Self::PrivacyPolicyUrl => f.write_str("privacyPolicyUrl"),
                Self::TvOsPrivacyPolicy => f.write_str("tvOsPrivacyPolicy"),
                Self::Description => f.write_str("description"),
                Self::Locale => f.write_str("locale"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsBetaAppLocalizationsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "feedbackEmail" => Ok(Self::FeedbackEmail),
                "marketingUrl" => Ok(Self::MarketingUrl),
                "privacyPolicyUrl" => Ok(Self::PrivacyPolicyUrl),
                "tvOsPrivacyPolicy" => Ok(Self::TvOsPrivacyPolicy),
                "description" => Ok(Self::Description),
                "locale" => Ok(Self::Locale),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsBetaAppLocalizationsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsBetaAppLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsBetaAppLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsBetaAppReviewDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "contactFirstName",
    ///    "contactLastName",
    ///    "contactPhone",
    ///    "contactEmail",
    ///    "demoAccountName",
    ///    "demoAccountPassword",
    ///    "demoAccountRequired",
    ///    "notes",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsBetaAppReviewDetailsItem {
        #[serde(rename = "contactFirstName")]
        ContactFirstName,
        #[serde(rename = "contactLastName")]
        ContactLastName,
        #[serde(rename = "contactPhone")]
        ContactPhone,
        #[serde(rename = "contactEmail")]
        ContactEmail,
        #[serde(rename = "demoAccountName")]
        DemoAccountName,
        #[serde(rename = "demoAccountPassword")]
        DemoAccountPassword,
        #[serde(rename = "demoAccountRequired")]
        DemoAccountRequired,
        #[serde(rename = "notes")]
        Notes,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsBetaAppReviewDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ContactFirstName => f.write_str("contactFirstName"),
                Self::ContactLastName => f.write_str("contactLastName"),
                Self::ContactPhone => f.write_str("contactPhone"),
                Self::ContactEmail => f.write_str("contactEmail"),
                Self::DemoAccountName => f.write_str("demoAccountName"),
                Self::DemoAccountPassword => f.write_str("demoAccountPassword"),
                Self::DemoAccountRequired => f.write_str("demoAccountRequired"),
                Self::Notes => f.write_str("notes"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsBetaAppReviewDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "contactFirstName" => Ok(Self::ContactFirstName),
                "contactLastName" => Ok(Self::ContactLastName),
                "contactPhone" => Ok(Self::ContactPhone),
                "contactEmail" => Ok(Self::ContactEmail),
                "demoAccountName" => Ok(Self::DemoAccountName),
                "demoAccountPassword" => Ok(Self::DemoAccountPassword),
                "demoAccountRequired" => Ok(Self::DemoAccountRequired),
                "notes" => Ok(Self::Notes),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsBetaAppReviewDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsBetaAppReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsBetaAppReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsBetaGroupsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "createdDate",
    ///    "isInternalGroup",
    ///    "hasAccessToAllBuilds",
    ///    "publicLinkEnabled",
    ///    "publicLinkId",
    ///    "publicLinkLimitEnabled",
    ///    "publicLinkLimit",
    ///    "publicLink",
    ///    "feedbackEnabled",
    ///    "iosBuildsAvailableForAppleSiliconMac",
    ///    "iosBuildsAvailableForAppleVision",
    ///    "app",
    ///    "builds",
    ///    "betaTesters",
    ///    "betaRecruitmentCriteria",
    ///    "betaRecruitmentCriterionCompatibleBuildCheck"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsBetaGroupsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "isInternalGroup")]
        IsInternalGroup,
        #[serde(rename = "hasAccessToAllBuilds")]
        HasAccessToAllBuilds,
        #[serde(rename = "publicLinkEnabled")]
        PublicLinkEnabled,
        #[serde(rename = "publicLinkId")]
        PublicLinkId,
        #[serde(rename = "publicLinkLimitEnabled")]
        PublicLinkLimitEnabled,
        #[serde(rename = "publicLinkLimit")]
        PublicLinkLimit,
        #[serde(rename = "publicLink")]
        PublicLink,
        #[serde(rename = "feedbackEnabled")]
        FeedbackEnabled,
        #[serde(rename = "iosBuildsAvailableForAppleSiliconMac")]
        IosBuildsAvailableForAppleSiliconMac,
        #[serde(rename = "iosBuildsAvailableForAppleVision")]
        IosBuildsAvailableForAppleVision,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaTesters")]
        BetaTesters,
        #[serde(rename = "betaRecruitmentCriteria")]
        BetaRecruitmentCriteria,
        #[serde(rename = "betaRecruitmentCriterionCompatibleBuildCheck")]
        BetaRecruitmentCriterionCompatibleBuildCheck,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsBetaGroupsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::IsInternalGroup => f.write_str("isInternalGroup"),
                Self::HasAccessToAllBuilds => f.write_str("hasAccessToAllBuilds"),
                Self::PublicLinkEnabled => f.write_str("publicLinkEnabled"),
                Self::PublicLinkId => f.write_str("publicLinkId"),
                Self::PublicLinkLimitEnabled => f.write_str("publicLinkLimitEnabled"),
                Self::PublicLinkLimit => f.write_str("publicLinkLimit"),
                Self::PublicLink => f.write_str("publicLink"),
                Self::FeedbackEnabled => f.write_str("feedbackEnabled"),
                Self::IosBuildsAvailableForAppleSiliconMac => {
                    f.write_str("iosBuildsAvailableForAppleSiliconMac")
                }
                Self::IosBuildsAvailableForAppleVision => {
                    f.write_str("iosBuildsAvailableForAppleVision")
                }
                Self::App => f.write_str("app"),
                Self::Builds => f.write_str("builds"),
                Self::BetaTesters => f.write_str("betaTesters"),
                Self::BetaRecruitmentCriteria => f.write_str("betaRecruitmentCriteria"),
                Self::BetaRecruitmentCriterionCompatibleBuildCheck => {
                    f.write_str("betaRecruitmentCriterionCompatibleBuildCheck")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsBetaGroupsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "createdDate" => Ok(Self::CreatedDate),
                "isInternalGroup" => Ok(Self::IsInternalGroup),
                "hasAccessToAllBuilds" => Ok(Self::HasAccessToAllBuilds),
                "publicLinkEnabled" => Ok(Self::PublicLinkEnabled),
                "publicLinkId" => Ok(Self::PublicLinkId),
                "publicLinkLimitEnabled" => Ok(Self::PublicLinkLimitEnabled),
                "publicLinkLimit" => Ok(Self::PublicLinkLimit),
                "publicLink" => Ok(Self::PublicLink),
                "feedbackEnabled" => Ok(Self::FeedbackEnabled),
                "iosBuildsAvailableForAppleSiliconMac" => {
                    Ok(Self::IosBuildsAvailableForAppleSiliconMac)
                }
                "iosBuildsAvailableForAppleVision" => Ok(Self::IosBuildsAvailableForAppleVision),
                "app" => Ok(Self::App),
                "builds" => Ok(Self::Builds),
                "betaTesters" => Ok(Self::BetaTesters),
                "betaRecruitmentCriteria" => Ok(Self::BetaRecruitmentCriteria),
                "betaRecruitmentCriterionCompatibleBuildCheck" => {
                    Ok(Self::BetaRecruitmentCriterionCompatibleBuildCheck)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsBetaGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsBetaGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsBetaGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsBetaLicenseAgreementsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "agreementText",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsBetaLicenseAgreementsItem {
        #[serde(rename = "agreementText")]
        AgreementText,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsBetaLicenseAgreementsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AgreementText => f.write_str("agreementText"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsBetaLicenseAgreementsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "agreementText" => Ok(Self::AgreementText),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsBetaLicenseAgreementsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsBetaLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsBetaLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsBuildIconsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "iconAsset",
    ///    "iconType",
    ///    "masked",
    ///    "name"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsBuildIconsItem {
        #[serde(rename = "iconAsset")]
        IconAsset,
        #[serde(rename = "iconType")]
        IconType,
        #[serde(rename = "masked")]
        Masked,
        #[serde(rename = "name")]
        Name,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsBuildIconsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::IconAsset => f.write_str("iconAsset"),
                Self::IconType => f.write_str("iconType"),
                Self::Masked => f.write_str("masked"),
                Self::Name => f.write_str("name"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsBuildIconsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "iconAsset" => Ok(Self::IconAsset),
                "iconType" => Ok(Self::IconType),
                "masked" => Ok(Self::Masked),
                "name" => Ok(Self::Name),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsBuildIconsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsBuildIconsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsBuildIconsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsBuildsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "version",
    ///    "uploadedDate",
    ///    "expirationDate",
    ///    "expired",
    ///    "minOsVersion",
    ///    "lsMinimumSystemVersion",
    ///    "computedMinMacOsVersion",
    ///    "computedMinVisionOsVersion",
    ///    "iconAssetToken",
    ///    "processingState",
    ///    "buildAudienceType",
    ///    "usesNonExemptEncryption",
    ///    "preReleaseVersion",
    ///    "individualTesters",
    ///    "betaGroups",
    ///    "betaBuildLocalizations",
    ///    "appEncryptionDeclaration",
    ///    "betaAppReviewSubmission",
    ///    "app",
    ///    "buildBetaDetail",
    ///    "appStoreVersion",
    ///    "icons",
    ///    "buildBundles",
    ///    "buildUpload",
    ///    "perfPowerMetrics",
    ///    "diagnosticSignatures"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsBuildsItem {
        #[serde(rename = "version")]
        Version,
        #[serde(rename = "uploadedDate")]
        UploadedDate,
        #[serde(rename = "expirationDate")]
        ExpirationDate,
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "minOsVersion")]
        MinOsVersion,
        #[serde(rename = "lsMinimumSystemVersion")]
        LsMinimumSystemVersion,
        #[serde(rename = "computedMinMacOsVersion")]
        ComputedMinMacOsVersion,
        #[serde(rename = "computedMinVisionOsVersion")]
        ComputedMinVisionOsVersion,
        #[serde(rename = "iconAssetToken")]
        IconAssetToken,
        #[serde(rename = "processingState")]
        ProcessingState,
        #[serde(rename = "buildAudienceType")]
        BuildAudienceType,
        #[serde(rename = "usesNonExemptEncryption")]
        UsesNonExemptEncryption,
        #[serde(rename = "preReleaseVersion")]
        PreReleaseVersion,
        #[serde(rename = "individualTesters")]
        IndividualTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "betaBuildLocalizations")]
        BetaBuildLocalizations,
        #[serde(rename = "appEncryptionDeclaration")]
        AppEncryptionDeclaration,
        #[serde(rename = "betaAppReviewSubmission")]
        BetaAppReviewSubmission,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "buildBetaDetail")]
        BuildBetaDetail,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "icons")]
        Icons,
        #[serde(rename = "buildBundles")]
        BuildBundles,
        #[serde(rename = "buildUpload")]
        BuildUpload,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "diagnosticSignatures")]
        DiagnosticSignatures,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsBuildsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Version => f.write_str("version"),
                Self::UploadedDate => f.write_str("uploadedDate"),
                Self::ExpirationDate => f.write_str("expirationDate"),
                Self::Expired => f.write_str("expired"),
                Self::MinOsVersion => f.write_str("minOsVersion"),
                Self::LsMinimumSystemVersion => f.write_str("lsMinimumSystemVersion"),
                Self::ComputedMinMacOsVersion => f.write_str("computedMinMacOsVersion"),
                Self::ComputedMinVisionOsVersion => f.write_str("computedMinVisionOsVersion"),
                Self::IconAssetToken => f.write_str("iconAssetToken"),
                Self::ProcessingState => f.write_str("processingState"),
                Self::BuildAudienceType => f.write_str("buildAudienceType"),
                Self::UsesNonExemptEncryption => f.write_str("usesNonExemptEncryption"),
                Self::PreReleaseVersion => f.write_str("preReleaseVersion"),
                Self::IndividualTesters => f.write_str("individualTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::BetaBuildLocalizations => f.write_str("betaBuildLocalizations"),
                Self::AppEncryptionDeclaration => f.write_str("appEncryptionDeclaration"),
                Self::BetaAppReviewSubmission => f.write_str("betaAppReviewSubmission"),
                Self::App => f.write_str("app"),
                Self::BuildBetaDetail => f.write_str("buildBetaDetail"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::Icons => f.write_str("icons"),
                Self::BuildBundles => f.write_str("buildBundles"),
                Self::BuildUpload => f.write_str("buildUpload"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::DiagnosticSignatures => f.write_str("diagnosticSignatures"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsBuildsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "version" => Ok(Self::Version),
                "uploadedDate" => Ok(Self::UploadedDate),
                "expirationDate" => Ok(Self::ExpirationDate),
                "expired" => Ok(Self::Expired),
                "minOsVersion" => Ok(Self::MinOsVersion),
                "lsMinimumSystemVersion" => Ok(Self::LsMinimumSystemVersion),
                "computedMinMacOsVersion" => Ok(Self::ComputedMinMacOsVersion),
                "computedMinVisionOsVersion" => Ok(Self::ComputedMinVisionOsVersion),
                "iconAssetToken" => Ok(Self::IconAssetToken),
                "processingState" => Ok(Self::ProcessingState),
                "buildAudienceType" => Ok(Self::BuildAudienceType),
                "usesNonExemptEncryption" => Ok(Self::UsesNonExemptEncryption),
                "preReleaseVersion" => Ok(Self::PreReleaseVersion),
                "individualTesters" => Ok(Self::IndividualTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "betaBuildLocalizations" => Ok(Self::BetaBuildLocalizations),
                "appEncryptionDeclaration" => Ok(Self::AppEncryptionDeclaration),
                "betaAppReviewSubmission" => Ok(Self::BetaAppReviewSubmission),
                "app" => Ok(Self::App),
                "buildBetaDetail" => Ok(Self::BuildBetaDetail),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "icons" => Ok(Self::Icons),
                "buildBundles" => Ok(Self::BuildBundles),
                "buildUpload" => Ok(Self::BuildUpload),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "diagnosticSignatures" => Ok(Self::DiagnosticSignatures),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsCiProductsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "createdDate",
    ///    "productType",
    ///    "app",
    ///    "bundleId",
    ///    "workflows",
    ///    "primaryRepositories",
    ///    "additionalRepositories",
    ///    "buildRuns"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsCiProductsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "productType")]
        ProductType,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "workflows")]
        Workflows,
        #[serde(rename = "primaryRepositories")]
        PrimaryRepositories,
        #[serde(rename = "additionalRepositories")]
        AdditionalRepositories,
        #[serde(rename = "buildRuns")]
        BuildRuns,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsCiProductsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::ProductType => f.write_str("productType"),
                Self::App => f.write_str("app"),
                Self::BundleId => f.write_str("bundleId"),
                Self::Workflows => f.write_str("workflows"),
                Self::PrimaryRepositories => f.write_str("primaryRepositories"),
                Self::AdditionalRepositories => f.write_str("additionalRepositories"),
                Self::BuildRuns => f.write_str("buildRuns"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsCiProductsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "createdDate" => Ok(Self::CreatedDate),
                "productType" => Ok(Self::ProductType),
                "app" => Ok(Self::App),
                "bundleId" => Ok(Self::BundleId),
                "workflows" => Ok(Self::Workflows),
                "primaryRepositories" => Ok(Self::PrimaryRepositories),
                "additionalRepositories" => Ok(Self::AdditionalRepositories),
                "buildRuns" => Ok(Self::BuildRuns),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsCiProductsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsCiProductsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsCiProductsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsEndUserLicenseAgreementsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "agreementText",
    ///    "app",
    ///    "territories"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsEndUserLicenseAgreementsItem {
        #[serde(rename = "agreementText")]
        AgreementText,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "territories")]
        Territories,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsEndUserLicenseAgreementsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AgreementText => f.write_str("agreementText"),
                Self::App => f.write_str("app"),
                Self::Territories => f.write_str("territories"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsEndUserLicenseAgreementsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "agreementText" => Ok(Self::AgreementText),
                "app" => Ok(Self::App),
                "territories" => Ok(Self::Territories),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsEndUserLicenseAgreementsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsEndUserLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsEndUserLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsGameCenterDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "arcadeEnabled",
    ///    "challengeEnabled",
    ///    "app",
    ///    "gameCenterAppVersions",
    ///    "gameCenterGroup",
    ///    "gameCenterLeaderboards",
    ///    "gameCenterLeaderboardsV2",
    ///    "gameCenterLeaderboardSets",
    ///    "gameCenterLeaderboardSetsV2",
    ///    "gameCenterAchievements",
    ///    "gameCenterAchievementsV2",
    ///    "gameCenterActivities",
    ///    "gameCenterChallenges",
    ///    "defaultLeaderboard",
    ///    "defaultLeaderboardV2",
    ///    "defaultGroupLeaderboard",
    ///    "defaultGroupLeaderboardV2",
    ///    "achievementReleases",
    ///    "activityReleases",
    ///    "challengeReleases",
    ///    "leaderboardReleases",
    ///    "leaderboardSetReleases",
    ///    "challengesMinimumPlatformVersions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsGameCenterDetailsItem {
        #[serde(rename = "arcadeEnabled")]
        ArcadeEnabled,
        #[serde(rename = "challengeEnabled")]
        ChallengeEnabled,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "gameCenterAppVersions")]
        GameCenterAppVersions,
        #[serde(rename = "gameCenterGroup")]
        GameCenterGroup,
        #[serde(rename = "gameCenterLeaderboards")]
        GameCenterLeaderboards,
        #[serde(rename = "gameCenterLeaderboardsV2")]
        GameCenterLeaderboardsV2,
        #[serde(rename = "gameCenterLeaderboardSets")]
        GameCenterLeaderboardSets,
        #[serde(rename = "gameCenterLeaderboardSetsV2")]
        GameCenterLeaderboardSetsV2,
        #[serde(rename = "gameCenterAchievements")]
        GameCenterAchievements,
        #[serde(rename = "gameCenterAchievementsV2")]
        GameCenterAchievementsV2,
        #[serde(rename = "gameCenterActivities")]
        GameCenterActivities,
        #[serde(rename = "gameCenterChallenges")]
        GameCenterChallenges,
        #[serde(rename = "defaultLeaderboard")]
        DefaultLeaderboard,
        #[serde(rename = "defaultLeaderboardV2")]
        DefaultLeaderboardV2,
        #[serde(rename = "defaultGroupLeaderboard")]
        DefaultGroupLeaderboard,
        #[serde(rename = "defaultGroupLeaderboardV2")]
        DefaultGroupLeaderboardV2,
        #[serde(rename = "achievementReleases")]
        AchievementReleases,
        #[serde(rename = "activityReleases")]
        ActivityReleases,
        #[serde(rename = "challengeReleases")]
        ChallengeReleases,
        #[serde(rename = "leaderboardReleases")]
        LeaderboardReleases,
        #[serde(rename = "leaderboardSetReleases")]
        LeaderboardSetReleases,
        #[serde(rename = "challengesMinimumPlatformVersions")]
        ChallengesMinimumPlatformVersions,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsGameCenterDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ArcadeEnabled => f.write_str("arcadeEnabled"),
                Self::ChallengeEnabled => f.write_str("challengeEnabled"),
                Self::App => f.write_str("app"),
                Self::GameCenterAppVersions => f.write_str("gameCenterAppVersions"),
                Self::GameCenterGroup => f.write_str("gameCenterGroup"),
                Self::GameCenterLeaderboards => f.write_str("gameCenterLeaderboards"),
                Self::GameCenterLeaderboardsV2 => f.write_str("gameCenterLeaderboardsV2"),
                Self::GameCenterLeaderboardSets => f.write_str("gameCenterLeaderboardSets"),
                Self::GameCenterLeaderboardSetsV2 => f.write_str("gameCenterLeaderboardSetsV2"),
                Self::GameCenterAchievements => f.write_str("gameCenterAchievements"),
                Self::GameCenterAchievementsV2 => f.write_str("gameCenterAchievementsV2"),
                Self::GameCenterActivities => f.write_str("gameCenterActivities"),
                Self::GameCenterChallenges => f.write_str("gameCenterChallenges"),
                Self::DefaultLeaderboard => f.write_str("defaultLeaderboard"),
                Self::DefaultLeaderboardV2 => f.write_str("defaultLeaderboardV2"),
                Self::DefaultGroupLeaderboard => f.write_str("defaultGroupLeaderboard"),
                Self::DefaultGroupLeaderboardV2 => f.write_str("defaultGroupLeaderboardV2"),
                Self::AchievementReleases => f.write_str("achievementReleases"),
                Self::ActivityReleases => f.write_str("activityReleases"),
                Self::ChallengeReleases => f.write_str("challengeReleases"),
                Self::LeaderboardReleases => f.write_str("leaderboardReleases"),
                Self::LeaderboardSetReleases => f.write_str("leaderboardSetReleases"),
                Self::ChallengesMinimumPlatformVersions => {
                    f.write_str("challengesMinimumPlatformVersions")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsGameCenterDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "arcadeEnabled" => Ok(Self::ArcadeEnabled),
                "challengeEnabled" => Ok(Self::ChallengeEnabled),
                "app" => Ok(Self::App),
                "gameCenterAppVersions" => Ok(Self::GameCenterAppVersions),
                "gameCenterGroup" => Ok(Self::GameCenterGroup),
                "gameCenterLeaderboards" => Ok(Self::GameCenterLeaderboards),
                "gameCenterLeaderboardsV2" => Ok(Self::GameCenterLeaderboardsV2),
                "gameCenterLeaderboardSets" => Ok(Self::GameCenterLeaderboardSets),
                "gameCenterLeaderboardSetsV2" => Ok(Self::GameCenterLeaderboardSetsV2),
                "gameCenterAchievements" => Ok(Self::GameCenterAchievements),
                "gameCenterAchievementsV2" => Ok(Self::GameCenterAchievementsV2),
                "gameCenterActivities" => Ok(Self::GameCenterActivities),
                "gameCenterChallenges" => Ok(Self::GameCenterChallenges),
                "defaultLeaderboard" => Ok(Self::DefaultLeaderboard),
                "defaultLeaderboardV2" => Ok(Self::DefaultLeaderboardV2),
                "defaultGroupLeaderboard" => Ok(Self::DefaultGroupLeaderboard),
                "defaultGroupLeaderboardV2" => Ok(Self::DefaultGroupLeaderboardV2),
                "achievementReleases" => Ok(Self::AchievementReleases),
                "activityReleases" => Ok(Self::ActivityReleases),
                "challengeReleases" => Ok(Self::ChallengeReleases),
                "leaderboardReleases" => Ok(Self::LeaderboardReleases),
                "leaderboardSetReleases" => Ok(Self::LeaderboardSetReleases),
                "challengesMinimumPlatformVersions" => Ok(Self::ChallengesMinimumPlatformVersions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsGameCenterDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsGameCenterDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsGameCenterDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsGameCenterEnabledVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "versionString",
    ///    "iconAsset",
    ///    "compatibleVersions",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsGameCenterEnabledVersionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "versionString")]
        VersionString,
        #[serde(rename = "iconAsset")]
        IconAsset,
        #[serde(rename = "compatibleVersions")]
        CompatibleVersions,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsGameCenterEnabledVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::VersionString => f.write_str("versionString"),
                Self::IconAsset => f.write_str("iconAsset"),
                Self::CompatibleVersions => f.write_str("compatibleVersions"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsGameCenterEnabledVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "versionString" => Ok(Self::VersionString),
                "iconAsset" => Ok(Self::IconAsset),
                "compatibleVersions" => Ok(Self::CompatibleVersions),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsGameCenterEnabledVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsGameCenterEnabledVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsGameCenterEnabledVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsInAppPurchasesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "referenceName",
    ///    "productId",
    ///    "inAppPurchaseType",
    ///    "state",
    ///    "apps",
    ///    "name",
    ///    "reviewNote",
    ///    "familySharable",
    ///    "contentHosting",
    ///    "inAppPurchaseLocalizations",
    ///    "pricePoints",
    ///    "content",
    ///    "appStoreReviewScreenshot",
    ///    "promotedPurchase",
    ///    "iapPriceSchedule",
    ///    "inAppPurchaseAvailability",
    ///    "images",
    ///    "offerCodes"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsInAppPurchasesItem {
        #[serde(rename = "referenceName")]
        ReferenceName,
        #[serde(rename = "productId")]
        ProductId,
        #[serde(rename = "inAppPurchaseType")]
        InAppPurchaseType,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "apps")]
        Apps,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "reviewNote")]
        ReviewNote,
        #[serde(rename = "familySharable")]
        FamilySharable,
        #[serde(rename = "contentHosting")]
        ContentHosting,
        #[serde(rename = "inAppPurchaseLocalizations")]
        InAppPurchaseLocalizations,
        #[serde(rename = "pricePoints")]
        PricePoints,
        #[serde(rename = "content")]
        Content,
        #[serde(rename = "appStoreReviewScreenshot")]
        AppStoreReviewScreenshot,
        #[serde(rename = "promotedPurchase")]
        PromotedPurchase,
        #[serde(rename = "iapPriceSchedule")]
        IapPriceSchedule,
        #[serde(rename = "inAppPurchaseAvailability")]
        InAppPurchaseAvailability,
        #[serde(rename = "images")]
        Images,
        #[serde(rename = "offerCodes")]
        OfferCodes,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsInAppPurchasesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReferenceName => f.write_str("referenceName"),
                Self::ProductId => f.write_str("productId"),
                Self::InAppPurchaseType => f.write_str("inAppPurchaseType"),
                Self::State => f.write_str("state"),
                Self::Apps => f.write_str("apps"),
                Self::Name => f.write_str("name"),
                Self::ReviewNote => f.write_str("reviewNote"),
                Self::FamilySharable => f.write_str("familySharable"),
                Self::ContentHosting => f.write_str("contentHosting"),
                Self::InAppPurchaseLocalizations => f.write_str("inAppPurchaseLocalizations"),
                Self::PricePoints => f.write_str("pricePoints"),
                Self::Content => f.write_str("content"),
                Self::AppStoreReviewScreenshot => f.write_str("appStoreReviewScreenshot"),
                Self::PromotedPurchase => f.write_str("promotedPurchase"),
                Self::IapPriceSchedule => f.write_str("iapPriceSchedule"),
                Self::InAppPurchaseAvailability => f.write_str("inAppPurchaseAvailability"),
                Self::Images => f.write_str("images"),
                Self::OfferCodes => f.write_str("offerCodes"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsInAppPurchasesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "referenceName" => Ok(Self::ReferenceName),
                "productId" => Ok(Self::ProductId),
                "inAppPurchaseType" => Ok(Self::InAppPurchaseType),
                "state" => Ok(Self::State),
                "apps" => Ok(Self::Apps),
                "name" => Ok(Self::Name),
                "reviewNote" => Ok(Self::ReviewNote),
                "familySharable" => Ok(Self::FamilySharable),
                "contentHosting" => Ok(Self::ContentHosting),
                "inAppPurchaseLocalizations" => Ok(Self::InAppPurchaseLocalizations),
                "pricePoints" => Ok(Self::PricePoints),
                "content" => Ok(Self::Content),
                "appStoreReviewScreenshot" => Ok(Self::AppStoreReviewScreenshot),
                "promotedPurchase" => Ok(Self::PromotedPurchase),
                "iapPriceSchedule" => Ok(Self::IapPriceSchedule),
                "inAppPurchaseAvailability" => Ok(Self::InAppPurchaseAvailability),
                "images" => Ok(Self::Images),
                "offerCodes" => Ok(Self::OfferCodes),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsInAppPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsInAppPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsInAppPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsPreReleaseVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "version",
    ///    "platform",
    ///    "builds",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsPreReleaseVersionsItem {
        #[serde(rename = "version")]
        Version,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsPreReleaseVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Version => f.write_str("version"),
                Self::Platform => f.write_str("platform"),
                Self::Builds => f.write_str("builds"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsPreReleaseVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "version" => Ok(Self::Version),
                "platform" => Ok(Self::Platform),
                "builds" => Ok(Self::Builds),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsPreReleaseVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsPreReleaseVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsPreReleaseVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsPromotedPurchasesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "visibleForAllUsers",
    ///    "enabled",
    ///    "state",
    ///    "inAppPurchaseV2",
    ///    "subscription"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsPromotedPurchasesItem {
        #[serde(rename = "visibleForAllUsers")]
        VisibleForAllUsers,
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "inAppPurchaseV2")]
        InAppPurchaseV2,
        #[serde(rename = "subscription")]
        Subscription,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsPromotedPurchasesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::VisibleForAllUsers => f.write_str("visibleForAllUsers"),
                Self::Enabled => f.write_str("enabled"),
                Self::State => f.write_str("state"),
                Self::InAppPurchaseV2 => f.write_str("inAppPurchaseV2"),
                Self::Subscription => f.write_str("subscription"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsPromotedPurchasesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "visibleForAllUsers" => Ok(Self::VisibleForAllUsers),
                "enabled" => Ok(Self::Enabled),
                "state" => Ok(Self::State),
                "inAppPurchaseV2" => Ok(Self::InAppPurchaseV2),
                "subscription" => Ok(Self::Subscription),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsPromotedPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsPromotedPurchasesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsPromotedPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsReviewSubmissionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "submittedDate",
    ///    "state",
    ///    "app",
    ///    "items",
    ///    "appStoreVersionForReview",
    ///    "submittedByActor",
    ///    "lastUpdatedByActor"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsReviewSubmissionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "submittedDate")]
        SubmittedDate,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "items")]
        Items,
        #[serde(rename = "appStoreVersionForReview")]
        AppStoreVersionForReview,
        #[serde(rename = "submittedByActor")]
        SubmittedByActor,
        #[serde(rename = "lastUpdatedByActor")]
        LastUpdatedByActor,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsReviewSubmissionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::SubmittedDate => f.write_str("submittedDate"),
                Self::State => f.write_str("state"),
                Self::App => f.write_str("app"),
                Self::Items => f.write_str("items"),
                Self::AppStoreVersionForReview => f.write_str("appStoreVersionForReview"),
                Self::SubmittedByActor => f.write_str("submittedByActor"),
                Self::LastUpdatedByActor => f.write_str("lastUpdatedByActor"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsReviewSubmissionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "submittedDate" => Ok(Self::SubmittedDate),
                "state" => Ok(Self::State),
                "app" => Ok(Self::App),
                "items" => Ok(Self::Items),
                "appStoreVersionForReview" => Ok(Self::AppStoreVersionForReview),
                "submittedByActor" => Ok(Self::SubmittedByActor),
                "lastUpdatedByActor" => Ok(Self::LastUpdatedByActor),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsReviewSubmissionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsReviewSubmissionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsReviewSubmissionsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsSubscriptionGracePeriodsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "optIn",
    ///    "sandboxOptIn",
    ///    "duration",
    ///    "renewalType"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsSubscriptionGracePeriodsItem {
        #[serde(rename = "optIn")]
        OptIn,
        #[serde(rename = "sandboxOptIn")]
        SandboxOptIn,
        #[serde(rename = "duration")]
        Duration,
        #[serde(rename = "renewalType")]
        RenewalType,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsSubscriptionGracePeriodsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::OptIn => f.write_str("optIn"),
                Self::SandboxOptIn => f.write_str("sandboxOptIn"),
                Self::Duration => f.write_str("duration"),
                Self::RenewalType => f.write_str("renewalType"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsSubscriptionGracePeriodsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "optIn" => Ok(Self::OptIn),
                "sandboxOptIn" => Ok(Self::SandboxOptIn),
                "duration" => Ok(Self::Duration),
                "renewalType" => Ok(Self::RenewalType),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsSubscriptionGracePeriodsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsSubscriptionGracePeriodsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsSubscriptionGracePeriodsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsSubscriptionGroupsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "referenceName",
    ///    "subscriptions",
    ///    "subscriptionGroupLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsSubscriptionGroupsItem {
        #[serde(rename = "referenceName")]
        ReferenceName,
        #[serde(rename = "subscriptions")]
        Subscriptions,
        #[serde(rename = "subscriptionGroupLocalizations")]
        SubscriptionGroupLocalizations,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsSubscriptionGroupsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReferenceName => f.write_str("referenceName"),
                Self::Subscriptions => f.write_str("subscriptions"),
                Self::SubscriptionGroupLocalizations => {
                    f.write_str("subscriptionGroupLocalizations")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsSubscriptionGroupsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "referenceName" => Ok(Self::ReferenceName),
                "subscriptions" => Ok(Self::Subscriptions),
                "subscriptionGroupLocalizations" => Ok(Self::SubscriptionGroupLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsSubscriptionGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsSubscriptionGroupsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsSubscriptionGroupsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceIncludeItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appEncryptionDeclarations",
    ///    "appStoreIcon",
    ///    "ciProduct",
    ///    "betaGroups",
    ///    "appStoreVersions",
    ///    "preReleaseVersions",
    ///    "betaAppLocalizations",
    ///    "builds",
    ///    "betaLicenseAgreement",
    ///    "betaAppReviewDetail",
    ///    "appInfos",
    ///    "appClips",
    ///    "endUserLicenseAgreement",
    ///    "inAppPurchases",
    ///    "subscriptionGroups",
    ///    "gameCenterEnabledVersions",
    ///    "appCustomProductPages",
    ///    "inAppPurchasesV2",
    ///    "promotedPurchases",
    ///    "appEvents",
    ///    "reviewSubmissions",
    ///    "subscriptionGracePeriod",
    ///    "gameCenterDetail",
    ///    "appStoreVersionExperimentsV2",
    ///    "androidToIosAppMappingDetails"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceIncludeItem {
        #[serde(rename = "appEncryptionDeclarations")]
        AppEncryptionDeclarations,
        #[serde(rename = "appStoreIcon")]
        AppStoreIcon,
        #[serde(rename = "ciProduct")]
        CiProduct,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
        #[serde(rename = "preReleaseVersions")]
        PreReleaseVersions,
        #[serde(rename = "betaAppLocalizations")]
        BetaAppLocalizations,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaLicenseAgreement")]
        BetaLicenseAgreement,
        #[serde(rename = "betaAppReviewDetail")]
        BetaAppReviewDetail,
        #[serde(rename = "appInfos")]
        AppInfos,
        #[serde(rename = "appClips")]
        AppClips,
        #[serde(rename = "endUserLicenseAgreement")]
        EndUserLicenseAgreement,
        #[serde(rename = "inAppPurchases")]
        InAppPurchases,
        #[serde(rename = "subscriptionGroups")]
        SubscriptionGroups,
        #[serde(rename = "gameCenterEnabledVersions")]
        GameCenterEnabledVersions,
        #[serde(rename = "appCustomProductPages")]
        AppCustomProductPages,
        #[serde(rename = "inAppPurchasesV2")]
        InAppPurchasesV2,
        #[serde(rename = "promotedPurchases")]
        PromotedPurchases,
        #[serde(rename = "appEvents")]
        AppEvents,
        #[serde(rename = "reviewSubmissions")]
        ReviewSubmissions,
        #[serde(rename = "subscriptionGracePeriod")]
        SubscriptionGracePeriod,
        #[serde(rename = "gameCenterDetail")]
        GameCenterDetail,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "androidToIosAppMappingDetails")]
        AndroidToIosAppMappingDetails,
    }

    impl ::std::fmt::Display for AppsGetInstanceIncludeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppEncryptionDeclarations => f.write_str("appEncryptionDeclarations"),
                Self::AppStoreIcon => f.write_str("appStoreIcon"),
                Self::CiProduct => f.write_str("ciProduct"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
                Self::PreReleaseVersions => f.write_str("preReleaseVersions"),
                Self::BetaAppLocalizations => f.write_str("betaAppLocalizations"),
                Self::Builds => f.write_str("builds"),
                Self::BetaLicenseAgreement => f.write_str("betaLicenseAgreement"),
                Self::BetaAppReviewDetail => f.write_str("betaAppReviewDetail"),
                Self::AppInfos => f.write_str("appInfos"),
                Self::AppClips => f.write_str("appClips"),
                Self::EndUserLicenseAgreement => f.write_str("endUserLicenseAgreement"),
                Self::InAppPurchases => f.write_str("inAppPurchases"),
                Self::SubscriptionGroups => f.write_str("subscriptionGroups"),
                Self::GameCenterEnabledVersions => f.write_str("gameCenterEnabledVersions"),
                Self::AppCustomProductPages => f.write_str("appCustomProductPages"),
                Self::InAppPurchasesV2 => f.write_str("inAppPurchasesV2"),
                Self::PromotedPurchases => f.write_str("promotedPurchases"),
                Self::AppEvents => f.write_str("appEvents"),
                Self::ReviewSubmissions => f.write_str("reviewSubmissions"),
                Self::SubscriptionGracePeriod => f.write_str("subscriptionGracePeriod"),
                Self::GameCenterDetail => f.write_str("gameCenterDetail"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AndroidToIosAppMappingDetails => f.write_str("androidToIosAppMappingDetails"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceIncludeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appEncryptionDeclarations" => Ok(Self::AppEncryptionDeclarations),
                "appStoreIcon" => Ok(Self::AppStoreIcon),
                "ciProduct" => Ok(Self::CiProduct),
                "betaGroups" => Ok(Self::BetaGroups),
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                "preReleaseVersions" => Ok(Self::PreReleaseVersions),
                "betaAppLocalizations" => Ok(Self::BetaAppLocalizations),
                "builds" => Ok(Self::Builds),
                "betaLicenseAgreement" => Ok(Self::BetaLicenseAgreement),
                "betaAppReviewDetail" => Ok(Self::BetaAppReviewDetail),
                "appInfos" => Ok(Self::AppInfos),
                "appClips" => Ok(Self::AppClips),
                "endUserLicenseAgreement" => Ok(Self::EndUserLicenseAgreement),
                "inAppPurchases" => Ok(Self::InAppPurchases),
                "subscriptionGroups" => Ok(Self::SubscriptionGroups),
                "gameCenterEnabledVersions" => Ok(Self::GameCenterEnabledVersions),
                "appCustomProductPages" => Ok(Self::AppCustomProductPages),
                "inAppPurchasesV2" => Ok(Self::InAppPurchasesV2),
                "promotedPurchases" => Ok(Self::PromotedPurchases),
                "appEvents" => Ok(Self::AppEvents),
                "reviewSubmissions" => Ok(Self::ReviewSubmissions),
                "subscriptionGracePeriod" => Ok(Self::SubscriptionGracePeriod),
                "gameCenterDetail" => Ok(Self::GameCenterDetail),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "androidToIosAppMappingDetails" => Ok(Self::AndroidToIosAppMappingDetails),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppsResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "links"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/App"
    ///      }
    ///    },
    ///    "included": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "$ref": "#/components/schemas/Build"
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/PagedDocumentLinks"
    ///    },
    ///    "meta": {
    ///      "$ref": "#/components/schemas/PagingInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppsResponse {
        pub data: ::std::vec::Vec<App>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub included: ::std::vec::Vec<Build>,
        pub links: PagedDocumentLinks,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub meta: ::std::option::Option<PagingInformation>,
    }

    ///`BrazilAgeRating`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "L",
    ///    "TEN",
    ///    "TWELVE",
    ///    "FOURTEEN",
    ///    "SIXTEEN",
    ///    "EIGHTEEN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum BrazilAgeRating {
        L,
        #[serde(rename = "TEN")]
        Ten,
        #[serde(rename = "TWELVE")]
        Twelve,
        #[serde(rename = "FOURTEEN")]
        Fourteen,
        #[serde(rename = "SIXTEEN")]
        Sixteen,
        #[serde(rename = "EIGHTEEN")]
        Eighteen,
    }

    impl ::std::fmt::Display for BrazilAgeRating {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::L => f.write_str("L"),
                Self::Ten => f.write_str("TEN"),
                Self::Twelve => f.write_str("TWELVE"),
                Self::Fourteen => f.write_str("FOURTEEN"),
                Self::Sixteen => f.write_str("SIXTEEN"),
                Self::Eighteen => f.write_str("EIGHTEEN"),
            }
        }
    }

    impl ::std::str::FromStr for BrazilAgeRating {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "L" => Ok(Self::L),
                "TEN" => Ok(Self::Ten),
                "TWELVE" => Ok(Self::Twelve),
                "FOURTEEN" => Ok(Self::Fourteen),
                "SIXTEEN" => Ok(Self::Sixteen),
                "EIGHTEEN" => Ok(Self::Eighteen),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BrazilAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BrazilAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BrazilAgeRating {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Build`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "Build",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "version": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "builds"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Build {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<BuildAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(rename = "type")]
        pub type_: BuildType,
    }

    ///`BuildAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "version": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BuildAttributes {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for BuildAttributes {
        fn default() -> Self {
            Self {
                version: Default::default(),
            }
        }
    }

    ///`BuildType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "builds"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum BuildType {
        #[serde(rename = "builds")]
        Builds,
    }

    impl ::std::fmt::Display for BuildType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Builds => f.write_str("builds"),
            }
        }
    }

    impl ::std::str::FromStr for BuildType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "builds" => Ok(Self::Builds),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BuildType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BuildType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BuildType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`DocumentLinks`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "self"
    ///  ],
    ///  "properties": {
    ///    "self": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DocumentLinks {
        #[serde(rename = "self")]
        pub self_: ::std::string::String,
    }

    ///`ErrorLinks`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "about": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    },
    ///    "associated": {
    ///      "oneOf": [
    ///        {
    ///          "type": "string",
    ///          "format": "uri-reference"
    ///        },
    ///        {
    ///          "type": "object",
    ///          "properties": {
    ///            "href": {
    ///              "type": "string",
    ///              "format": "uri-reference"
    ///            },
    ///            "meta": {
    ///              "type": "object",
    ///              "properties": {
    ///                "source": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorLinks {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub about: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub associated: ::std::option::Option<ErrorLinksAssociated>,
    }

    impl ::std::default::Default for ErrorLinks {
        fn default() -> Self {
            Self {
                about: Default::default(),
                associated: Default::default(),
            }
        }
    }

    ///`ErrorLinksAssociated`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "href": {
    ///          "type": "string",
    ///          "format": "uri-reference"
    ///        },
    ///        "meta": {
    ///          "type": "object",
    ///          "properties": {
    ///            "source": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ErrorLinksAssociated {
        UriReference(::std::string::String),
        Object {
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            href: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            meta: ::std::option::Option<ErrorLinksAssociatedObjectMeta>,
        },
    }

    ///`ErrorLinksAssociatedObjectMeta`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "source": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorLinksAssociatedObjectMeta {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ErrorLinksAssociatedObjectMeta {
        fn default() -> Self {
            Self {
                source: Default::default(),
            }
        }
    }

    ///`ErrorResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "errors": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "code",
    ///          "detail",
    ///          "status",
    ///          "title"
    ///        ],
    ///        "properties": {
    ///          "code": {
    ///            "type": "string"
    ///          },
    ///          "detail": {
    ///            "type": "string"
    ///          },
    ///          "id": {
    ///            "type": "string"
    ///          },
    ///          "links": {
    ///            "$ref": "#/components/schemas/ErrorLinks"
    ///          },
    ///          "meta": {
    ///            "type": "object",
    ///            "additionalProperties": {}
    ///          },
    ///          "source": {
    ///            "oneOf": [
    ///              {
    ///                "$ref": "#/components/schemas/ErrorSourcePointer"
    ///              },
    ///              {
    ///                "$ref": "#/components/schemas/ErrorSourceParameter"
    ///              }
    ///            ]
    ///          },
    ///          "status": {
    ///            "type": "string"
    ///          },
    ///          "title": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub errors: ::std::vec::Vec<ErrorResponseErrorsItem>,
    }

    impl ::std::default::Default for ErrorResponse {
        fn default() -> Self {
            Self {
                errors: Default::default(),
            }
        }
    }

    ///`ErrorResponseErrorsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "detail",
    ///    "status",
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "detail": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ErrorLinks"
    ///    },
    ///    "meta": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "source": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ErrorSourcePointer"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/ErrorSourceParameter"
    ///        }
    ///      ]
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorResponseErrorsItem {
        pub code: ::std::string::String,
        pub detail: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ErrorLinks>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<ErrorResponseErrorsItemSource>,
        pub status: ::std::string::String,
        pub title: ::std::string::String,
    }

    ///`ErrorResponseErrorsItemSource`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/ErrorSourcePointer"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/ErrorSourceParameter"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ErrorResponseErrorsItemSource {
        Pointer(ErrorSourcePointer),
        Parameter(ErrorSourceParameter),
    }

    impl ::std::convert::From<ErrorSourcePointer> for ErrorResponseErrorsItemSource {
        fn from(value: ErrorSourcePointer) -> Self {
            Self::Pointer(value)
        }
    }

    impl ::std::convert::From<ErrorSourceParameter> for ErrorResponseErrorsItemSource {
        fn from(value: ErrorSourceParameter) -> Self {
            Self::Parameter(value)
        }
    }

    ///`ErrorSourceParameter`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "Parameter",
    ///  "type": "object",
    ///  "required": [
    ///    "parameter"
    ///  ],
    ///  "properties": {
    ///    "parameter": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorSourceParameter {
        pub parameter: ::std::string::String,
    }

    ///`ErrorSourcePointer`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonPointer",
    ///  "type": "object",
    ///  "required": [
    ///    "pointer"
    ///  ],
    ///  "properties": {
    ///    "pointer": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorSourcePointer {
        pub pointer: ::std::string::String,
    }

    ///`KidsAgeBand`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "FIVE_AND_UNDER",
    ///    "SIX_TO_EIGHT",
    ///    "NINE_TO_ELEVEN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum KidsAgeBand {
        #[serde(rename = "FIVE_AND_UNDER")]
        FiveAndUnder,
        #[serde(rename = "SIX_TO_EIGHT")]
        SixToEight,
        #[serde(rename = "NINE_TO_ELEVEN")]
        NineToEleven,
    }

    impl ::std::fmt::Display for KidsAgeBand {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::FiveAndUnder => f.write_str("FIVE_AND_UNDER"),
                Self::SixToEight => f.write_str("SIX_TO_EIGHT"),
                Self::NineToEleven => f.write_str("NINE_TO_ELEVEN"),
            }
        }
    }

    impl ::std::str::FromStr for KidsAgeBand {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "FIVE_AND_UNDER" => Ok(Self::FiveAndUnder),
                "SIX_TO_EIGHT" => Ok(Self::SixToEight),
                "NINE_TO_ELEVEN" => Ok(Self::NineToEleven),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for KidsAgeBand {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for KidsAgeBand {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for KidsAgeBand {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PagedDocumentLinks`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "self"
    ///  ],
    ///  "properties": {
    ///    "first": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    },
    ///    "next": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    },
    ///    "self": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PagedDocumentLinks {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub first: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next: ::std::option::Option<::std::string::String>,
        #[serde(rename = "self")]
        pub self_: ::std::string::String,
    }

    ///`PagingInformation`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "paging"
    ///  ],
    ///  "properties": {
    ///    "paging": {
    ///      "type": "object",
    ///      "required": [
    ///        "limit"
    ///      ],
    ///      "properties": {
    ///        "limit": {
    ///          "type": "integer"
    ///        },
    ///        "nextCursor": {
    ///          "type": "string"
    ///        },
    ///        "total": {
    ///          "type": "integer"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PagingInformation {
        pub paging: PagingInformationPaging,
    }

    ///`PagingInformationPaging`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "limit"
    ///  ],
    ///  "properties": {
    ///    "limit": {
    ///      "type": "integer"
    ///    },
    ///    "nextCursor": {
    ///      "type": "string"
    ///    },
    ///    "total": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PagingInformationPaging {
        pub limit: i64,
        #[serde(
            rename = "nextCursor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_cursor: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total: ::std::option::Option<i64>,
    }

    ///`Platform`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "IOS",
    ///    "MAC_OS",
    ///    "TV_OS",
    ///    "VISION_OS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum Platform {
        #[serde(rename = "IOS")]
        Ios,
        #[serde(rename = "MAC_OS")]
        MacOs,
        #[serde(rename = "TV_OS")]
        TvOs,
        #[serde(rename = "VISION_OS")]
        VisionOs,
    }

    impl ::std::fmt::Display for Platform {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ios => f.write_str("IOS"),
                Self::MacOs => f.write_str("MAC_OS"),
                Self::TvOs => f.write_str("TV_OS"),
                Self::VisionOs => f.write_str("VISION_OS"),
            }
        }
    }

    impl ::std::str::FromStr for Platform {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "IOS" => Ok(Self::Ios),
                "MAC_OS" => Ok(Self::MacOs),
                "TV_OS" => Ok(Self::TvOs),
                "VISION_OS" => Ok(Self::VisionOs),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for Platform {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Platform {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Platform {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PreviewType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "IPHONE_67",
    ///    "IPHONE_61",
    ///    "IPHONE_65",
    ///    "IPHONE_58",
    ///    "IPHONE_55",
    ///    "IPHONE_47",
    ///    "IPHONE_40",
    ///    "IPHONE_35",
    ///    "IPAD_PRO_3GEN_129",
    ///    "IPAD_PRO_3GEN_11",
    ///    "IPAD_PRO_129",
    ///    "IPAD_105",
    ///    "IPAD_97",
    ///    "DESKTOP",
    ///    "APPLE_TV",
    ///    "APPLE_VISION_PRO"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PreviewType {
        #[serde(rename = "IPHONE_67")]
        Iphone67,
        #[serde(rename = "IPHONE_61")]
        Iphone61,
        #[serde(rename = "IPHONE_65")]
        Iphone65,
        #[serde(rename = "IPHONE_58")]
        Iphone58,
        #[serde(rename = "IPHONE_55")]
        Iphone55,
        #[serde(rename = "IPHONE_47")]
        Iphone47,
        #[serde(rename = "IPHONE_40")]
        Iphone40,
        #[serde(rename = "IPHONE_35")]
        Iphone35,
        #[serde(rename = "IPAD_PRO_3GEN_129")]
        IpadPro3gen129,
        #[serde(rename = "IPAD_PRO_3GEN_11")]
        IpadPro3gen11,
        #[serde(rename = "IPAD_PRO_129")]
        IpadPro129,
        #[serde(rename = "IPAD_105")]
        Ipad105,
        #[serde(rename = "IPAD_97")]
        Ipad97,
        #[serde(rename = "DESKTOP")]
        Desktop,
        #[serde(rename = "APPLE_TV")]
        AppleTv,
        #[serde(rename = "APPLE_VISION_PRO")]
        AppleVisionPro,
    }

    impl ::std::fmt::Display for PreviewType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Iphone67 => f.write_str("IPHONE_67"),
                Self::Iphone61 => f.write_str("IPHONE_61"),
                Self::Iphone65 => f.write_str("IPHONE_65"),
                Self::Iphone58 => f.write_str("IPHONE_58"),
                Self::Iphone55 => f.write_str("IPHONE_55"),
                Self::Iphone47 => f.write_str("IPHONE_47"),
                Self::Iphone40 => f.write_str("IPHONE_40"),
                Self::Iphone35 => f.write_str("IPHONE_35"),
                Self::IpadPro3gen129 => f.write_str("IPAD_PRO_3GEN_129"),
                Self::IpadPro3gen11 => f.write_str("IPAD_PRO_3GEN_11"),
                Self::IpadPro129 => f.write_str("IPAD_PRO_129"),
                Self::Ipad105 => f.write_str("IPAD_105"),
                Self::Ipad97 => f.write_str("IPAD_97"),
                Self::Desktop => f.write_str("DESKTOP"),
                Self::AppleTv => f.write_str("APPLE_TV"),
                Self::AppleVisionPro => f.write_str("APPLE_VISION_PRO"),
            }
        }
    }

    impl ::std::str::FromStr for PreviewType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "IPHONE_67" => Ok(Self::Iphone67),
                "IPHONE_61" => Ok(Self::Iphone61),
                "IPHONE_65" => Ok(Self::Iphone65),
                "IPHONE_58" => Ok(Self::Iphone58),
                "IPHONE_55" => Ok(Self::Iphone55),
                "IPHONE_47" => Ok(Self::Iphone47),
                "IPHONE_40" => Ok(Self::Iphone40),
                "IPHONE_35" => Ok(Self::Iphone35),
                "IPAD_PRO_3GEN_129" => Ok(Self::IpadPro3gen129),
                "IPAD_PRO_3GEN_11" => Ok(Self::IpadPro3gen11),
                "IPAD_PRO_129" => Ok(Self::IpadPro129),
                "IPAD_105" => Ok(Self::Ipad105),
                "IPAD_97" => Ok(Self::Ipad97),
                "DESKTOP" => Ok(Self::Desktop),
                "APPLE_TV" => Ok(Self::AppleTv),
                "APPLE_VISION_PRO" => Ok(Self::AppleVisionPro),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PreviewType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PreviewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PreviewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`RelationshipLinks`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "related": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    },
    ///    "self": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RelationshipLinks {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub related: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "self",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub self_: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for RelationshipLinks {
        fn default() -> Self {
            Self {
                related: Default::default(),
                self_: Default::default(),
            }
        }
    }

    ///`ResourceLinks`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "self": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ResourceLinks {
        #[serde(
            rename = "self",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub self_: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ResourceLinks {
        fn default() -> Self {
            Self {
                self_: Default::default(),
            }
        }
    }

    ///`ScreenshotDisplayType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "APP_IPHONE_67",
    ///    "APP_IPHONE_61",
    ///    "APP_IPHONE_65",
    ///    "APP_IPHONE_58",
    ///    "APP_IPHONE_55",
    ///    "APP_IPHONE_47",
    ///    "APP_IPHONE_40",
    ///    "APP_IPHONE_35",
    ///    "APP_IPAD_PRO_3GEN_129",
    ///    "APP_IPAD_PRO_3GEN_11",
    ///    "APP_IPAD_PRO_129",
    ///    "APP_IPAD_105",
    ///    "APP_IPAD_97",
    ///    "APP_DESKTOP",
    ///    "APP_WATCH_ULTRA",
    ///    "APP_WATCH_SERIES_10",
    ///    "APP_WATCH_SERIES_7",
    ///    "APP_WATCH_SERIES_4",
    ///    "APP_WATCH_SERIES_3",
    ///    "APP_APPLE_TV",
    ///    "APP_APPLE_VISION_PRO",
    ///    "IMESSAGE_APP_IPHONE_67",
    ///    "IMESSAGE_APP_IPHONE_61",
    ///    "IMESSAGE_APP_IPHONE_65",
    ///    "IMESSAGE_APP_IPHONE_58",
    ///    "IMESSAGE_APP_IPHONE_55",
    ///    "IMESSAGE_APP_IPHONE_47",
    ///    "IMESSAGE_APP_IPHONE_40",
    ///    "IMESSAGE_APP_IPAD_PRO_3GEN_129",
    ///    "IMESSAGE_APP_IPAD_PRO_3GEN_11",
    ///    "IMESSAGE_APP_IPAD_PRO_129",
    ///    "IMESSAGE_APP_IPAD_105",
    ///    "IMESSAGE_APP_IPAD_97"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ScreenshotDisplayType {
        #[serde(rename = "APP_IPHONE_67")]
        AppIphone67,
        #[serde(rename = "APP_IPHONE_61")]
        AppIphone61,
        #[serde(rename = "APP_IPHONE_65")]
        AppIphone65,
        #[serde(rename = "APP_IPHONE_58")]
        AppIphone58,
        #[serde(rename = "APP_IPHONE_55")]
        AppIphone55,
        #[serde(rename = "APP_IPHONE_47")]
        AppIphone47,
        #[serde(rename = "APP_IPHONE_40")]
        AppIphone40,
        #[serde(rename = "APP_IPHONE_35")]
        AppIphone35,
        #[serde(rename = "APP_IPAD_PRO_3GEN_129")]
        AppIpadPro3gen129,
        #[serde(rename = "APP_IPAD_PRO_3GEN_11")]
        AppIpadPro3gen11,
        #[serde(rename = "APP_IPAD_PRO_129")]
        AppIpadPro129,
        #[serde(rename = "APP_IPAD_105")]
        AppIpad105,
        #[serde(rename = "APP_IPAD_97")]
        AppIpad97,
        #[serde(rename = "APP_DESKTOP")]
        AppDesktop,
        #[serde(rename = "APP_WATCH_ULTRA")]
        AppWatchUltra,
        #[serde(rename = "APP_WATCH_SERIES_10")]
        AppWatchSeries10,
        #[serde(rename = "APP_WATCH_SERIES_7")]
        AppWatchSeries7,
        #[serde(rename = "APP_WATCH_SERIES_4")]
        AppWatchSeries4,
        #[serde(rename = "APP_WATCH_SERIES_3")]
        AppWatchSeries3,
        #[serde(rename = "APP_APPLE_TV")]
        AppAppleTv,
        #[serde(rename = "APP_APPLE_VISION_PRO")]
        AppAppleVisionPro,
        #[serde(rename = "IMESSAGE_APP_IPHONE_67")]
        ImessageAppIphone67,
        #[serde(rename = "IMESSAGE_APP_IPHONE_61")]
        ImessageAppIphone61,
        #[serde(rename = "IMESSAGE_APP_IPHONE_65")]
        ImessageAppIphone65,
        #[serde(rename = "IMESSAGE_APP_IPHONE_58")]
        ImessageAppIphone58,
        #[serde(rename = "IMESSAGE_APP_IPHONE_55")]
        ImessageAppIphone55,
        #[serde(rename = "IMESSAGE_APP_IPHONE_47")]
        ImessageAppIphone47,
        #[serde(rename = "IMESSAGE_APP_IPHONE_40")]
        ImessageAppIphone40,
        #[serde(rename = "IMESSAGE_APP_IPAD_PRO_3GEN_129")]
        ImessageAppIpadPro3gen129,
        #[serde(rename = "IMESSAGE_APP_IPAD_PRO_3GEN_11")]
        ImessageAppIpadPro3gen11,
        #[serde(rename = "IMESSAGE_APP_IPAD_PRO_129")]
        ImessageAppIpadPro129,
        #[serde(rename = "IMESSAGE_APP_IPAD_105")]
        ImessageAppIpad105,
        #[serde(rename = "IMESSAGE_APP_IPAD_97")]
        ImessageAppIpad97,
    }

    impl ::std::fmt::Display for ScreenshotDisplayType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppIphone67 => f.write_str("APP_IPHONE_67"),
                Self::AppIphone61 => f.write_str("APP_IPHONE_61"),
                Self::AppIphone65 => f.write_str("APP_IPHONE_65"),
                Self::AppIphone58 => f.write_str("APP_IPHONE_58"),
                Self::AppIphone55 => f.write_str("APP_IPHONE_55"),
                Self::AppIphone47 => f.write_str("APP_IPHONE_47"),
                Self::AppIphone40 => f.write_str("APP_IPHONE_40"),
                Self::AppIphone35 => f.write_str("APP_IPHONE_35"),
                Self::AppIpadPro3gen129 => f.write_str("APP_IPAD_PRO_3GEN_129"),
                Self::AppIpadPro3gen11 => f.write_str("APP_IPAD_PRO_3GEN_11"),
                Self::AppIpadPro129 => f.write_str("APP_IPAD_PRO_129"),
                Self::AppIpad105 => f.write_str("APP_IPAD_105"),
                Self::AppIpad97 => f.write_str("APP_IPAD_97"),
                Self::AppDesktop => f.write_str("APP_DESKTOP"),
                Self::AppWatchUltra => f.write_str("APP_WATCH_ULTRA"),
                Self::AppWatchSeries10 => f.write_str("APP_WATCH_SERIES_10"),
                Self::AppWatchSeries7 => f.write_str("APP_WATCH_SERIES_7"),
                Self::AppWatchSeries4 => f.write_str("APP_WATCH_SERIES_4"),
                Self::AppWatchSeries3 => f.write_str("APP_WATCH_SERIES_3"),
                Self::AppAppleTv => f.write_str("APP_APPLE_TV"),
                Self::AppAppleVisionPro => f.write_str("APP_APPLE_VISION_PRO"),
                Self::ImessageAppIphone67 => f.write_str("IMESSAGE_APP_IPHONE_67"),
                Self::ImessageAppIphone61 => f.write_str("IMESSAGE_APP_IPHONE_61"),
                Self::ImessageAppIphone65 => f.write_str("IMESSAGE_APP_IPHONE_65"),
                Self::ImessageAppIphone58 => f.write_str("IMESSAGE_APP_IPHONE_58"),
                Self::ImessageAppIphone55 => f.write_str("IMESSAGE_APP_IPHONE_55"),
                Self::ImessageAppIphone47 => f.write_str("IMESSAGE_APP_IPHONE_47"),
                Self::ImessageAppIphone40 => f.write_str("IMESSAGE_APP_IPHONE_40"),
                Self::ImessageAppIpadPro3gen129 => f.write_str("IMESSAGE_APP_IPAD_PRO_3GEN_129"),
                Self::ImessageAppIpadPro3gen11 => f.write_str("IMESSAGE_APP_IPAD_PRO_3GEN_11"),
                Self::ImessageAppIpadPro129 => f.write_str("IMESSAGE_APP_IPAD_PRO_129"),
                Self::ImessageAppIpad105 => f.write_str("IMESSAGE_APP_IPAD_105"),
                Self::ImessageAppIpad97 => f.write_str("IMESSAGE_APP_IPAD_97"),
            }
        }
    }

    impl ::std::str::FromStr for ScreenshotDisplayType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "APP_IPHONE_67" => Ok(Self::AppIphone67),
                "APP_IPHONE_61" => Ok(Self::AppIphone61),
                "APP_IPHONE_65" => Ok(Self::AppIphone65),
                "APP_IPHONE_58" => Ok(Self::AppIphone58),
                "APP_IPHONE_55" => Ok(Self::AppIphone55),
                "APP_IPHONE_47" => Ok(Self::AppIphone47),
                "APP_IPHONE_40" => Ok(Self::AppIphone40),
                "APP_IPHONE_35" => Ok(Self::AppIphone35),
                "APP_IPAD_PRO_3GEN_129" => Ok(Self::AppIpadPro3gen129),
                "APP_IPAD_PRO_3GEN_11" => Ok(Self::AppIpadPro3gen11),
                "APP_IPAD_PRO_129" => Ok(Self::AppIpadPro129),
                "APP_IPAD_105" => Ok(Self::AppIpad105),
                "APP_IPAD_97" => Ok(Self::AppIpad97),
                "APP_DESKTOP" => Ok(Self::AppDesktop),
                "APP_WATCH_ULTRA" => Ok(Self::AppWatchUltra),
                "APP_WATCH_SERIES_10" => Ok(Self::AppWatchSeries10),
                "APP_WATCH_SERIES_7" => Ok(Self::AppWatchSeries7),
                "APP_WATCH_SERIES_4" => Ok(Self::AppWatchSeries4),
                "APP_WATCH_SERIES_3" => Ok(Self::AppWatchSeries3),
                "APP_APPLE_TV" => Ok(Self::AppAppleTv),
                "APP_APPLE_VISION_PRO" => Ok(Self::AppAppleVisionPro),
                "IMESSAGE_APP_IPHONE_67" => Ok(Self::ImessageAppIphone67),
                "IMESSAGE_APP_IPHONE_61" => Ok(Self::ImessageAppIphone61),
                "IMESSAGE_APP_IPHONE_65" => Ok(Self::ImessageAppIphone65),
                "IMESSAGE_APP_IPHONE_58" => Ok(Self::ImessageAppIphone58),
                "IMESSAGE_APP_IPHONE_55" => Ok(Self::ImessageAppIphone55),
                "IMESSAGE_APP_IPHONE_47" => Ok(Self::ImessageAppIphone47),
                "IMESSAGE_APP_IPHONE_40" => Ok(Self::ImessageAppIphone40),
                "IMESSAGE_APP_IPAD_PRO_3GEN_129" => Ok(Self::ImessageAppIpadPro3gen129),
                "IMESSAGE_APP_IPAD_PRO_3GEN_11" => Ok(Self::ImessageAppIpadPro3gen11),
                "IMESSAGE_APP_IPAD_PRO_129" => Ok(Self::ImessageAppIpadPro129),
                "IMESSAGE_APP_IPAD_105" => Ok(Self::ImessageAppIpad105),
                "IMESSAGE_APP_IPAD_97" => Ok(Self::ImessageAppIpad97),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ScreenshotDisplayType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ScreenshotDisplayType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ScreenshotDisplayType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`SubscriptionStatusUrlVersion`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "V1",
    ///    "V2"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum SubscriptionStatusUrlVersion {
        V1,
        V2,
    }

    impl ::std::fmt::Display for SubscriptionStatusUrlVersion {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::V1 => f.write_str("V1"),
                Self::V2 => f.write_str("V2"),
            }
        }
    }

    impl ::std::str::FromStr for SubscriptionStatusUrlVersion {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "V1" => Ok(Self::V1),
                "V2" => Ok(Self::V2),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SubscriptionStatusUrlVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SubscriptionStatusUrlVersion {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SubscriptionStatusUrlVersion {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
}

#[derive(Clone, Debug)]
///Client for App Store Connect API
///
///Version: 4.4
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "4.4"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    ///Sends a `GET` request to `/v1/appInfoLocalizations/{id}`
    ///
    ///Arguments:
    /// - `id`: the id of the requested resource
    /// - `fields_app_info_localizations`: the fields to include for returned
    ///   resources of type appInfoLocalizations
    /// - `fields_app_infos`: the fields to include for returned resources of
    ///   type appInfos
    /// - `include`: comma-separated list of relationships to include
    pub async fn app_info_localizations_get_instance<'a>(
        &'a self,
        id: &'a str,
        fields_app_info_localizations: Option<
            &'a ::std::vec::Vec<
                types::AppInfoLocalizationsGetInstanceFieldsAppInfoLocalizationsItem,
            >,
        >,
        fields_app_infos: Option<
            &'a ::std::vec::Vec<types::AppInfoLocalizationsGetInstanceFieldsAppInfosItem>,
        >,
        include: Option<&'a ::std::vec::Vec<types::AppInfoLocalizationsGetInstanceIncludeItem>>,
    ) -> Result<ResponseValue<types::AppInfoLocalizationResponse>, Error<types::ErrorResponse>>
    {
        let url = format!(
            "{}/v1/appInfoLocalizations/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "fields[appInfoLocalizations]",
                &fields_app_info_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appInfos]",
                &fields_app_infos,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "app_info_localizations_get_instance",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PATCH` request to `/v1/appInfoLocalizations/{id}`
    ///
    ///Arguments:
    /// - `id`: the id of the requested resource
    /// - `body`: AppInfoLocalization representation
    pub async fn app_info_localizations_update_instance<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::AppInfoLocalizationUpdateRequest,
    ) -> Result<ResponseValue<types::AppInfoLocalizationResponse>, Error<types::ErrorResponse>>
    {
        let url = format!(
            "{}/v1/appInfoLocalizations/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "app_info_localizations_update_instance",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            409u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            422u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/appStoreVersionLocalizations/{id}`
    ///
    ///Arguments:
    /// - `id`: the id of the requested resource
    /// - `fields_app_preview_sets`: the fields to include for returned
    ///   resources of type appPreviewSets
    /// - `fields_app_screenshot_sets`: the fields to include for returned
    ///   resources of type appScreenshotSets
    /// - `fields_app_store_version_localizations`: the fields to include for
    ///   returned resources of type appStoreVersionLocalizations
    /// - `fields_app_store_versions`: the fields to include for returned
    ///   resources of type appStoreVersions
    /// - `include`: comma-separated list of relationships to include
    /// - `limit_app_preview_sets`: maximum number of related appPreviewSets
    ///   returned (when they are included)
    /// - `limit_app_screenshot_sets`: maximum number of related
    ///   appScreenshotSets returned (when they are included)
    /// - `limit_search_keywords`: maximum number of related searchKeywords
    ///   returned (when they are included)
    pub async fn app_store_version_localizations_get_instance<'a>(
        &'a self,
        id: &'a str,
        fields_app_preview_sets: Option<
            &'a ::std::vec::Vec<
                types::AppStoreVersionLocalizationsGetInstanceFieldsAppPreviewSetsItem,
            >,
        >,
        fields_app_screenshot_sets: Option<
            &'a ::std::vec::Vec<
                types::AppStoreVersionLocalizationsGetInstanceFieldsAppScreenshotSetsItem,
            >,
        >,
        fields_app_store_version_localizations : Option < & 'a :: std :: vec :: Vec < types :: AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionLocalizationsItem > >,
        fields_app_store_versions: Option<
            &'a ::std::vec::Vec<
                types::AppStoreVersionLocalizationsGetInstanceFieldsAppStoreVersionsItem,
            >,
        >,
        include: Option<
            &'a ::std::vec::Vec<types::AppStoreVersionLocalizationsGetInstanceIncludeItem>,
        >,
        limit_app_preview_sets: Option<i64>,
        limit_app_screenshot_sets: Option<i64>,
        limit_search_keywords: Option<i64>,
    ) -> Result<
        ResponseValue<types::AppStoreVersionLocalizationResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/v1/appStoreVersionLocalizations/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "fields[appPreviewSets]",
                &fields_app_preview_sets,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appScreenshotSets]",
                &fields_app_screenshot_sets,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionLocalizations]",
                &fields_app_store_version_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersions]",
                &fields_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new(
                "limit[appPreviewSets]",
                &limit_app_preview_sets,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appScreenshotSets]",
                &limit_app_screenshot_sets,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[searchKeywords]",
                &limit_search_keywords,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "app_store_version_localizations_get_instance",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PATCH` request to `/v1/appStoreVersionLocalizations/{id}`
    ///
    ///Arguments:
    /// - `id`: the id of the requested resource
    /// - `body`: AppStoreVersionLocalization representation
    pub async fn app_store_version_localizations_update_instance<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::AppStoreVersionLocalizationUpdateRequest,
    ) -> Result<
        ResponseValue<types::AppStoreVersionLocalizationResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/v1/appStoreVersionLocalizations/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "app_store_version_localizations_update_instance",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            409u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            422u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/apps`
    ///
    ///Arguments:
    /// - `exists_game_center_enabled_versions`: filter by existence or
    ///   non-existence of related 'gameCenterEnabledVersions'
    /// - `fields_android_to_ios_app_mapping_details`: the fields to include for
    ///   returned resources of type androidToIosAppMappingDetails
    /// - `fields_app_clips`: the fields to include for returned resources of
    ///   type appClips
    /// - `fields_app_custom_product_pages`: the fields to include for returned
    ///   resources of type appCustomProductPages
    /// - `fields_app_encryption_declarations`: the fields to include for
    ///   returned resources of type appEncryptionDeclarations
    /// - `fields_app_events`: the fields to include for returned resources of
    ///   type appEvents
    /// - `fields_app_infos`: the fields to include for returned resources of
    ///   type appInfos
    /// - `fields_app_store_version_experiments`: the fields to include for
    ///   returned resources of type appStoreVersionExperiments
    /// - `fields_app_store_versions`: the fields to include for returned
    ///   resources of type appStoreVersions
    /// - `fields_apps`: the fields to include for returned resources of type
    ///   apps
    /// - `fields_beta_app_localizations`: the fields to include for returned
    ///   resources of type betaAppLocalizations
    /// - `fields_beta_app_review_details`: the fields to include for returned
    ///   resources of type betaAppReviewDetails
    /// - `fields_beta_groups`: the fields to include for returned resources of
    ///   type betaGroups
    /// - `fields_beta_license_agreements`: the fields to include for returned
    ///   resources of type betaLicenseAgreements
    /// - `fields_build_icons`: the fields to include for returned resources of
    ///   type buildIcons
    /// - `fields_builds`: the fields to include for returned resources of type
    ///   builds
    /// - `fields_ci_products`: the fields to include for returned resources of
    ///   type ciProducts
    /// - `fields_end_user_license_agreements`: the fields to include for
    ///   returned resources of type endUserLicenseAgreements
    /// - `fields_game_center_details`: the fields to include for returned
    ///   resources of type gameCenterDetails
    /// - `fields_game_center_enabled_versions`: the fields to include for
    ///   returned resources of type gameCenterEnabledVersions
    /// - `fields_in_app_purchases`: the fields to include for returned
    ///   resources of type inAppPurchases
    /// - `fields_pre_release_versions`: the fields to include for returned
    ///   resources of type preReleaseVersions
    /// - `fields_promoted_purchases`: the fields to include for returned
    ///   resources of type promotedPurchases
    /// - `fields_review_submissions`: the fields to include for returned
    ///   resources of type reviewSubmissions
    /// - `fields_subscription_grace_periods`: the fields to include for
    ///   returned resources of type subscriptionGracePeriods
    /// - `fields_subscription_groups`: the fields to include for returned
    ///   resources of type subscriptionGroups
    /// - `filter_app_store_versions_app_store_state`: filter by attribute
    ///   'appStoreVersions.appStoreState'
    /// - `filter_app_store_versions_app_version_state`: filter by attribute
    ///   'appStoreVersions.appVersionState'
    /// - `filter_app_store_versions_platform`: filter by attribute
    ///   'appStoreVersions.platform'
    /// - `filter_app_store_versions`: filter by id(s) of related
    ///   'appStoreVersions'
    /// - `filter_bundle_id`: filter by attribute 'bundleId'
    /// - `filter_id`: filter by id(s)
    /// - `filter_name`: filter by attribute 'name'
    /// - `filter_review_submissions_platform`: filter by attribute
    ///   'reviewSubmissions.platform'
    /// - `filter_review_submissions_state`: filter by attribute
    ///   'reviewSubmissions.state'
    /// - `filter_sku`: filter by attribute 'sku'
    /// - `include`: comma-separated list of relationships to include
    /// - `limit`: maximum resources per page
    /// - `limit_android_to_ios_app_mapping_details`: maximum number of related
    ///   androidToIosAppMappingDetails returned (when they are included)
    /// - `limit_app_clips`: maximum number of related appClips returned (when
    ///   they are included)
    /// - `limit_app_custom_product_pages`: maximum number of related
    ///   appCustomProductPages returned (when they are included)
    /// - `limit_app_encryption_declarations`: maximum number of related
    ///   appEncryptionDeclarations returned (when they are included)
    /// - `limit_app_events`: maximum number of related appEvents returned (when
    ///   they are included)
    /// - `limit_app_infos`: maximum number of related appInfos returned (when
    ///   they are included)
    /// - `limit_app_store_version_experiments_v2`: maximum number of related
    ///   appStoreVersionExperimentsV2 returned (when they are included)
    /// - `limit_app_store_versions`: maximum number of related appStoreVersions
    ///   returned (when they are included)
    /// - `limit_beta_app_localizations`: maximum number of related
    ///   betaAppLocalizations returned (when they are included)
    /// - `limit_beta_groups`: maximum number of related betaGroups returned
    ///   (when they are included)
    /// - `limit_builds`: maximum number of related builds returned (when they
    ///   are included)
    /// - `limit_game_center_enabled_versions`: maximum number of related
    ///   gameCenterEnabledVersions returned (when they are included)
    /// - `limit_in_app_purchases_v2`: maximum number of related
    ///   inAppPurchasesV2 returned (when they are included)
    /// - `limit_in_app_purchases`: maximum number of related inAppPurchases
    ///   returned (when they are included)
    /// - `limit_pre_release_versions`: maximum number of related
    ///   preReleaseVersions returned (when they are included)
    /// - `limit_promoted_purchases`: maximum number of related
    ///   promotedPurchases returned (when they are included)
    /// - `limit_review_submissions`: maximum number of related
    ///   reviewSubmissions returned (when they are included)
    /// - `limit_subscription_groups`: maximum number of related
    ///   subscriptionGroups returned (when they are included)
    /// - `sort`: comma-separated list of sort expressions; resources will be
    ///   sorted as specified
    pub async fn apps_get_collection<'a>(
        &'a self,
        exists_game_center_enabled_versions: Option<bool>,
        fields_android_to_ios_app_mapping_details: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem>,
        >,
        fields_app_clips: Option<&'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppClipsItem>>,
        fields_app_custom_product_pages: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppCustomProductPagesItem>,
        >,
        fields_app_encryption_declarations: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppEncryptionDeclarationsItem>,
        >,
        fields_app_events: Option<&'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppEventsItem>>,
        fields_app_infos: Option<&'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppInfosItem>>,
        fields_app_store_version_experiments: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppStoreVersionExperimentsItem>,
        >,
        fields_app_store_versions: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppStoreVersionsItem>,
        >,
        fields_apps: Option<&'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppsItem>>,
        fields_beta_app_localizations: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsBetaAppLocalizationsItem>,
        >,
        fields_beta_app_review_details: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsBetaAppReviewDetailsItem>,
        >,
        fields_beta_groups: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsBetaGroupsItem>,
        >,
        fields_beta_license_agreements: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsBetaLicenseAgreementsItem>,
        >,
        fields_build_icons: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsBuildIconsItem>,
        >,
        fields_builds: Option<&'a ::std::vec::Vec<types::AppsGetCollectionFieldsBuildsItem>>,
        fields_ci_products: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsCiProductsItem>,
        >,
        fields_end_user_license_agreements: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsEndUserLicenseAgreementsItem>,
        >,
        fields_game_center_details: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsGameCenterDetailsItem>,
        >,
        fields_game_center_enabled_versions: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsGameCenterEnabledVersionsItem>,
        >,
        fields_in_app_purchases: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsInAppPurchasesItem>,
        >,
        fields_pre_release_versions: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsPreReleaseVersionsItem>,
        >,
        fields_promoted_purchases: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsPromotedPurchasesItem>,
        >,
        fields_review_submissions: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsReviewSubmissionsItem>,
        >,
        fields_subscription_grace_periods: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsSubscriptionGracePeriodsItem>,
        >,
        fields_subscription_groups: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsSubscriptionGroupsItem>,
        >,
        filter_app_store_versions_app_store_state: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem>,
        >,
        filter_app_store_versions_app_version_state: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem>,
        >,
        filter_app_store_versions_platform: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFilterAppStoreVersionsPlatformItem>,
        >,
        filter_app_store_versions: Option<&'a ::std::vec::Vec<::std::string::String>>,
        filter_bundle_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        filter_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        filter_name: Option<&'a ::std::vec::Vec<::std::string::String>>,
        filter_review_submissions_platform: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFilterReviewSubmissionsPlatformItem>,
        >,
        filter_review_submissions_state: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFilterReviewSubmissionsStateItem>,
        >,
        filter_sku: Option<&'a ::std::vec::Vec<::std::string::String>>,
        include: Option<&'a ::std::vec::Vec<types::AppsGetCollectionIncludeItem>>,
        limit: Option<i64>,
        limit_android_to_ios_app_mapping_details: Option<i64>,
        limit_app_clips: Option<i64>,
        limit_app_custom_product_pages: Option<i64>,
        limit_app_encryption_declarations: Option<i64>,
        limit_app_events: Option<i64>,
        limit_app_infos: Option<i64>,
        limit_app_store_version_experiments_v2: Option<i64>,
        limit_app_store_versions: Option<i64>,
        limit_beta_app_localizations: Option<i64>,
        limit_beta_groups: Option<i64>,
        limit_builds: Option<i64>,
        limit_game_center_enabled_versions: Option<i64>,
        limit_in_app_purchases_v2: Option<i64>,
        limit_in_app_purchases: Option<i64>,
        limit_pre_release_versions: Option<i64>,
        limit_promoted_purchases: Option<i64>,
        limit_review_submissions: Option<i64>,
        limit_subscription_groups: Option<i64>,
        sort: Option<&'a ::std::vec::Vec<types::AppsGetCollectionSortItem>>,
    ) -> Result<ResponseValue<types::AppsResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/v1/apps", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "exists[gameCenterEnabledVersions]",
                &exists_game_center_enabled_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[androidToIosAppMappingDetails]",
                &fields_android_to_ios_app_mapping_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appClips]",
                &fields_app_clips,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appCustomProductPages]",
                &fields_app_custom_product_pages,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appEncryptionDeclarations]",
                &fields_app_encryption_declarations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appEvents]",
                &fields_app_events,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appInfos]",
                &fields_app_infos,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionExperiments]",
                &fields_app_store_version_experiments,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersions]",
                &fields_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[apps]",
                &fields_apps,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaAppLocalizations]",
                &fields_beta_app_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaAppReviewDetails]",
                &fields_beta_app_review_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaGroups]",
                &fields_beta_groups,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaLicenseAgreements]",
                &fields_beta_license_agreements,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[buildIcons]",
                &fields_build_icons,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[builds]",
                &fields_builds,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[ciProducts]",
                &fields_ci_products,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[endUserLicenseAgreements]",
                &fields_end_user_license_agreements,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[gameCenterDetails]",
                &fields_game_center_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[gameCenterEnabledVersions]",
                &fields_game_center_enabled_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[inAppPurchases]",
                &fields_in_app_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[preReleaseVersions]",
                &fields_pre_release_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[promotedPurchases]",
                &fields_promoted_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[reviewSubmissions]",
                &fields_review_submissions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[subscriptionGracePeriods]",
                &fields_subscription_grace_periods,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[subscriptionGroups]",
                &fields_subscription_groups,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[appStoreVersions.appStoreState]",
                &filter_app_store_versions_app_store_state,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[appStoreVersions.appVersionState]",
                &filter_app_store_versions_app_version_state,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[appStoreVersions.platform]",
                &filter_app_store_versions_platform,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[appStoreVersions]",
                &filter_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[bundleId]",
                &filter_bundle_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[id]",
                &filter_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[name]",
                &filter_name,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[reviewSubmissions.platform]",
                &filter_review_submissions_platform,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[reviewSubmissions.state]",
                &filter_review_submissions_state,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[sku]",
                &filter_sku,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "limit[androidToIosAppMappingDetails]",
                &limit_android_to_ios_app_mapping_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appClips]",
                &limit_app_clips,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appCustomProductPages]",
                &limit_app_custom_product_pages,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appEncryptionDeclarations]",
                &limit_app_encryption_declarations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appEvents]",
                &limit_app_events,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appInfos]",
                &limit_app_infos,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersionExperimentsV2]",
                &limit_app_store_version_experiments_v2,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersions]",
                &limit_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[betaAppLocalizations]",
                &limit_beta_app_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[betaGroups]",
                &limit_beta_groups,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[builds]",
                &limit_builds,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[gameCenterEnabledVersions]",
                &limit_game_center_enabled_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[inAppPurchasesV2]",
                &limit_in_app_purchases_v2,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[inAppPurchases]",
                &limit_in_app_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[preReleaseVersions]",
                &limit_pre_release_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[promotedPurchases]",
                &limit_promoted_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[reviewSubmissions]",
                &limit_review_submissions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[subscriptionGroups]",
                &limit_subscription_groups,
            ))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "apps_get_collection",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/apps/{id}`
    ///
    ///Arguments:
    /// - `id`: the id of the requested resource
    /// - `fields_android_to_ios_app_mapping_details`: the fields to include for
    ///   returned resources of type androidToIosAppMappingDetails
    /// - `fields_app_clips`: the fields to include for returned resources of
    ///   type appClips
    /// - `fields_app_custom_product_pages`: the fields to include for returned
    ///   resources of type appCustomProductPages
    /// - `fields_app_encryption_declarations`: the fields to include for
    ///   returned resources of type appEncryptionDeclarations
    /// - `fields_app_events`: the fields to include for returned resources of
    ///   type appEvents
    /// - `fields_app_infos`: the fields to include for returned resources of
    ///   type appInfos
    /// - `fields_app_store_version_experiments`: the fields to include for
    ///   returned resources of type appStoreVersionExperiments
    /// - `fields_app_store_versions`: the fields to include for returned
    ///   resources of type appStoreVersions
    /// - `fields_apps`: the fields to include for returned resources of type
    ///   apps
    /// - `fields_beta_app_localizations`: the fields to include for returned
    ///   resources of type betaAppLocalizations
    /// - `fields_beta_app_review_details`: the fields to include for returned
    ///   resources of type betaAppReviewDetails
    /// - `fields_beta_groups`: the fields to include for returned resources of
    ///   type betaGroups
    /// - `fields_beta_license_agreements`: the fields to include for returned
    ///   resources of type betaLicenseAgreements
    /// - `fields_build_icons`: the fields to include for returned resources of
    ///   type buildIcons
    /// - `fields_builds`: the fields to include for returned resources of type
    ///   builds
    /// - `fields_ci_products`: the fields to include for returned resources of
    ///   type ciProducts
    /// - `fields_end_user_license_agreements`: the fields to include for
    ///   returned resources of type endUserLicenseAgreements
    /// - `fields_game_center_details`: the fields to include for returned
    ///   resources of type gameCenterDetails
    /// - `fields_game_center_enabled_versions`: the fields to include for
    ///   returned resources of type gameCenterEnabledVersions
    /// - `fields_in_app_purchases`: the fields to include for returned
    ///   resources of type inAppPurchases
    /// - `fields_pre_release_versions`: the fields to include for returned
    ///   resources of type preReleaseVersions
    /// - `fields_promoted_purchases`: the fields to include for returned
    ///   resources of type promotedPurchases
    /// - `fields_review_submissions`: the fields to include for returned
    ///   resources of type reviewSubmissions
    /// - `fields_subscription_grace_periods`: the fields to include for
    ///   returned resources of type subscriptionGracePeriods
    /// - `fields_subscription_groups`: the fields to include for returned
    ///   resources of type subscriptionGroups
    /// - `include`: comma-separated list of relationships to include
    /// - `limit_android_to_ios_app_mapping_details`: maximum number of related
    ///   androidToIosAppMappingDetails returned (when they are included)
    /// - `limit_app_clips`: maximum number of related appClips returned (when
    ///   they are included)
    /// - `limit_app_custom_product_pages`: maximum number of related
    ///   appCustomProductPages returned (when they are included)
    /// - `limit_app_encryption_declarations`: maximum number of related
    ///   appEncryptionDeclarations returned (when they are included)
    /// - `limit_app_events`: maximum number of related appEvents returned (when
    ///   they are included)
    /// - `limit_app_infos`: maximum number of related appInfos returned (when
    ///   they are included)
    /// - `limit_app_store_version_experiments_v2`: maximum number of related
    ///   appStoreVersionExperimentsV2 returned (when they are included)
    /// - `limit_app_store_versions`: maximum number of related appStoreVersions
    ///   returned (when they are included)
    /// - `limit_beta_app_localizations`: maximum number of related
    ///   betaAppLocalizations returned (when they are included)
    /// - `limit_beta_groups`: maximum number of related betaGroups returned
    ///   (when they are included)
    /// - `limit_builds`: maximum number of related builds returned (when they
    ///   are included)
    /// - `limit_game_center_enabled_versions`: maximum number of related
    ///   gameCenterEnabledVersions returned (when they are included)
    /// - `limit_in_app_purchases_v2`: maximum number of related
    ///   inAppPurchasesV2 returned (when they are included)
    /// - `limit_in_app_purchases`: maximum number of related inAppPurchases
    ///   returned (when they are included)
    /// - `limit_pre_release_versions`: maximum number of related
    ///   preReleaseVersions returned (when they are included)
    /// - `limit_promoted_purchases`: maximum number of related
    ///   promotedPurchases returned (when they are included)
    /// - `limit_review_submissions`: maximum number of related
    ///   reviewSubmissions returned (when they are included)
    /// - `limit_subscription_groups`: maximum number of related
    ///   subscriptionGroups returned (when they are included)
    pub async fn apps_get_instance<'a>(
        &'a self,
        id: &'a str,
        fields_android_to_ios_app_mapping_details: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem>,
        >,
        fields_app_clips: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppClipsItem>>,
        fields_app_custom_product_pages: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppCustomProductPagesItem>,
        >,
        fields_app_encryption_declarations: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppEncryptionDeclarationsItem>,
        >,
        fields_app_events: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppEventsItem>>,
        fields_app_infos: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppInfosItem>>,
        fields_app_store_version_experiments: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppStoreVersionExperimentsItem>,
        >,
        fields_app_store_versions: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppStoreVersionsItem>,
        >,
        fields_apps: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppsItem>>,
        fields_beta_app_localizations: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsBetaAppLocalizationsItem>,
        >,
        fields_beta_app_review_details: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsBetaAppReviewDetailsItem>,
        >,
        fields_beta_groups: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsBetaGroupsItem>>,
        fields_beta_license_agreements: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsBetaLicenseAgreementsItem>,
        >,
        fields_build_icons: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsBuildIconsItem>>,
        fields_builds: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsBuildsItem>>,
        fields_ci_products: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsCiProductsItem>>,
        fields_end_user_license_agreements: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsEndUserLicenseAgreementsItem>,
        >,
        fields_game_center_details: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsGameCenterDetailsItem>,
        >,
        fields_game_center_enabled_versions: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsGameCenterEnabledVersionsItem>,
        >,
        fields_in_app_purchases: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsInAppPurchasesItem>,
        >,
        fields_pre_release_versions: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsPreReleaseVersionsItem>,
        >,
        fields_promoted_purchases: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsPromotedPurchasesItem>,
        >,
        fields_review_submissions: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsReviewSubmissionsItem>,
        >,
        fields_subscription_grace_periods: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsSubscriptionGracePeriodsItem>,
        >,
        fields_subscription_groups: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsSubscriptionGroupsItem>,
        >,
        include: Option<&'a ::std::vec::Vec<types::AppsGetInstanceIncludeItem>>,
        limit_android_to_ios_app_mapping_details: Option<i64>,
        limit_app_clips: Option<i64>,
        limit_app_custom_product_pages: Option<i64>,
        limit_app_encryption_declarations: Option<i64>,
        limit_app_events: Option<i64>,
        limit_app_infos: Option<i64>,
        limit_app_store_version_experiments_v2: Option<i64>,
        limit_app_store_versions: Option<i64>,
        limit_beta_app_localizations: Option<i64>,
        limit_beta_groups: Option<i64>,
        limit_builds: Option<i64>,
        limit_game_center_enabled_versions: Option<i64>,
        limit_in_app_purchases_v2: Option<i64>,
        limit_in_app_purchases: Option<i64>,
        limit_pre_release_versions: Option<i64>,
        limit_promoted_purchases: Option<i64>,
        limit_review_submissions: Option<i64>,
        limit_subscription_groups: Option<i64>,
    ) -> Result<ResponseValue<types::AppResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/v1/apps/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "fields[androidToIosAppMappingDetails]",
                &fields_android_to_ios_app_mapping_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appClips]",
                &fields_app_clips,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appCustomProductPages]",
                &fields_app_custom_product_pages,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appEncryptionDeclarations]",
                &fields_app_encryption_declarations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appEvents]",
                &fields_app_events,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appInfos]",
                &fields_app_infos,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionExperiments]",
                &fields_app_store_version_experiments,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersions]",
                &fields_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[apps]",
                &fields_apps,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaAppLocalizations]",
                &fields_beta_app_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaAppReviewDetails]",
                &fields_beta_app_review_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaGroups]",
                &fields_beta_groups,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaLicenseAgreements]",
                &fields_beta_license_agreements,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[buildIcons]",
                &fields_build_icons,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[builds]",
                &fields_builds,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[ciProducts]",
                &fields_ci_products,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[endUserLicenseAgreements]",
                &fields_end_user_license_agreements,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[gameCenterDetails]",
                &fields_game_center_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[gameCenterEnabledVersions]",
                &fields_game_center_enabled_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[inAppPurchases]",
                &fields_in_app_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[preReleaseVersions]",
                &fields_pre_release_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[promotedPurchases]",
                &fields_promoted_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[reviewSubmissions]",
                &fields_review_submissions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[subscriptionGracePeriods]",
                &fields_subscription_grace_periods,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[subscriptionGroups]",
                &fields_subscription_groups,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new(
                "limit[androidToIosAppMappingDetails]",
                &limit_android_to_ios_app_mapping_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appClips]",
                &limit_app_clips,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appCustomProductPages]",
                &limit_app_custom_product_pages,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appEncryptionDeclarations]",
                &limit_app_encryption_declarations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appEvents]",
                &limit_app_events,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appInfos]",
                &limit_app_infos,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersionExperimentsV2]",
                &limit_app_store_version_experiments_v2,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersions]",
                &limit_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[betaAppLocalizations]",
                &limit_beta_app_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[betaGroups]",
                &limit_beta_groups,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[builds]",
                &limit_builds,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[gameCenterEnabledVersions]",
                &limit_game_center_enabled_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[inAppPurchasesV2]",
                &limit_in_app_purchases_v2,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[inAppPurchases]",
                &limit_in_app_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[preReleaseVersions]",
                &limit_pre_release_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[promotedPurchases]",
                &limit_promoted_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[reviewSubmissions]",
                &limit_review_submissions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[subscriptionGroups]",
                &limit_subscription_groups,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "apps_get_instance",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PATCH` request to `/v1/apps/{id}`
    ///
    ///Arguments:
    /// - `id`: the id of the requested resource
    /// - `body`: App representation
    pub async fn apps_update_instance<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::AppUpdateRequest,
    ) -> Result<ResponseValue<types::AppResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/v1/apps/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "apps_update_instance",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            409u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            422u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/appInfos/{id}/appInfoLocalizations`
    ///
    ///Arguments:
    /// - `id`: the id of the requested resource
    /// - `fields_app_info_localizations`: the fields to include for returned
    ///   resources of type appInfoLocalizations
    /// - `fields_app_infos`: the fields to include for returned resources of
    ///   type appInfos
    /// - `filter_locale`: filter by attribute 'locale'
    /// - `include`: comma-separated list of relationships to include
    /// - `limit`: maximum resources per page
    pub async fn app_infos_app_info_localizations_get_to_many_related<'a>(
        &'a self,
        id: &'a str,
        fields_app_info_localizations: Option<
            &'a ::std::vec::Vec<
                types::AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfoLocalizationsItem,
            >,
        >,
        fields_app_infos: Option<
            &'a ::std::vec::Vec<
                types::AppInfosAppInfoLocalizationsGetToManyRelatedFieldsAppInfosItem,
            >,
        >,
        filter_locale: Option<&'a ::std::vec::Vec<::std::string::String>>,
        include: Option<
            &'a ::std::vec::Vec<types::AppInfosAppInfoLocalizationsGetToManyRelatedIncludeItem>,
        >,
        limit: Option<i64>,
    ) -> Result<ResponseValue<types::AppInfoLocalizationsResponse>, Error<types::ErrorResponse>>
    {
        let url = format!(
            "{}/v1/appInfos/{}/appInfoLocalizations",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "fields[appInfoLocalizations]",
                &fields_app_info_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appInfos]",
                &fields_app_infos,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[locale]",
                &filter_locale,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "app_infos_app_info_localizations_get_to_many_related",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/v1/appStoreVersions/{id}/appStoreVersionLocalizations`
    ///
    ///Arguments:
    /// - `id`: the id of the requested resource
    /// - `fields_app_keywords`: the fields to include for returned resources of
    ///   type appKeywords
    /// - `fields_app_preview_sets`: the fields to include for returned
    ///   resources of type appPreviewSets
    /// - `fields_app_screenshot_sets`: the fields to include for returned
    ///   resources of type appScreenshotSets
    /// - `fields_app_store_version_localizations`: the fields to include for
    ///   returned resources of type appStoreVersionLocalizations
    /// - `fields_app_store_versions`: the fields to include for returned
    ///   resources of type appStoreVersions
    /// - `filter_locale`: filter by attribute 'locale'
    /// - `include`: comma-separated list of relationships to include
    /// - `limit`: maximum resources per page
    /// - `limit_app_preview_sets`: maximum number of related appPreviewSets
    ///   returned (when they are included)
    /// - `limit_app_screenshot_sets`: maximum number of related
    ///   appScreenshotSets returned (when they are included)
    /// - `limit_search_keywords`: maximum number of related searchKeywords
    ///   returned (when they are included)
    pub async fn app_store_versions_app_store_version_localizations_get_to_many_related<'a>(
        &'a self,
        id: &'a str,
        fields_app_keywords: Option<&'a ::std::vec::Vec<::std::string::String>>,
        fields_app_preview_sets : Option < & 'a :: std :: vec :: Vec < types :: AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppPreviewSetsItem > >,
        fields_app_screenshot_sets : Option < & 'a :: std :: vec :: Vec < types :: AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppScreenshotSetsItem > >,
        fields_app_store_version_localizations : Option < & 'a :: std :: vec :: Vec < types :: AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem > >,
        fields_app_store_versions : Option < & 'a :: std :: vec :: Vec < types :: AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedFieldsAppStoreVersionsItem > >,
        filter_locale: Option<&'a ::std::vec::Vec<::std::string::String>>,
        include: Option<
            &'a ::std::vec::Vec<
                types::AppStoreVersionsAppStoreVersionLocalizationsGetToManyRelatedIncludeItem,
            >,
        >,
        limit: Option<i64>,
        limit_app_preview_sets: Option<i64>,
        limit_app_screenshot_sets: Option<i64>,
        limit_search_keywords: Option<i64>,
    ) -> Result<
        ResponseValue<types::AppStoreVersionLocalizationsResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/v1/appStoreVersions/{}/appStoreVersionLocalizations",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "fields[appKeywords]",
                &fields_app_keywords,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appPreviewSets]",
                &fields_app_preview_sets,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appScreenshotSets]",
                &fields_app_screenshot_sets,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionLocalizations]",
                &fields_app_store_version_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersions]",
                &fields_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[locale]",
                &filter_locale,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "limit[appPreviewSets]",
                &limit_app_preview_sets,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appScreenshotSets]",
                &limit_app_screenshot_sets,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[searchKeywords]",
                &limit_search_keywords,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "app_store_versions_app_store_version_localizations_get_to_many_related",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/apps/{id}/appInfos`
    ///
    ///Arguments:
    /// - `id`: the id of the requested resource
    /// - `fields_age_rating_declarations`: the fields to include for returned
    ///   resources of type ageRatingDeclarations
    /// - `fields_app_categories`: the fields to include for returned resources
    ///   of type appCategories
    /// - `fields_app_info_localizations`: the fields to include for returned
    ///   resources of type appInfoLocalizations
    /// - `fields_app_infos`: the fields to include for returned resources of
    ///   type appInfos
    /// - `fields_apps`: the fields to include for returned resources of type
    ///   apps
    /// - `include`: comma-separated list of relationships to include
    /// - `limit`: maximum resources per page
    /// - `limit_app_info_localizations`: maximum number of related
    ///   appInfoLocalizations returned (when they are included)
    pub async fn apps_app_infos_get_to_many_related<'a>(
        &'a self,
        id: &'a str,
        fields_age_rating_declarations: Option<
            &'a ::std::vec::Vec<types::AppsAppInfosGetToManyRelatedFieldsAgeRatingDeclarationsItem>,
        >,
        fields_app_categories: Option<
            &'a ::std::vec::Vec<types::AppsAppInfosGetToManyRelatedFieldsAppCategoriesItem>,
        >,
        fields_app_info_localizations: Option<
            &'a ::std::vec::Vec<types::AppsAppInfosGetToManyRelatedFieldsAppInfoLocalizationsItem>,
        >,
        fields_app_infos: Option<
            &'a ::std::vec::Vec<types::AppsAppInfosGetToManyRelatedFieldsAppInfosItem>,
        >,
        fields_apps: Option<&'a ::std::vec::Vec<types::AppsAppInfosGetToManyRelatedFieldsAppsItem>>,
        include: Option<&'a ::std::vec::Vec<types::AppsAppInfosGetToManyRelatedIncludeItem>>,
        limit: Option<i64>,
        limit_app_info_localizations: Option<i64>,
    ) -> Result<ResponseValue<types::AppInfosResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/v1/apps/{}/appInfos",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "fields[ageRatingDeclarations]",
                &fields_age_rating_declarations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appCategories]",
                &fields_app_categories,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appInfoLocalizations]",
                &fields_app_info_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appInfos]",
                &fields_app_infos,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[apps]",
                &fields_apps,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "limit[appInfoLocalizations]",
                &limit_app_info_localizations,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "apps_app_infos_get_to_many_related",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/apps/{id}/appStoreVersions`
    ///
    ///Arguments:
    /// - `id`: the id of the requested resource
    /// - `fields_alternative_distribution_packages`: the fields to include for
    ///   returned resources of type alternativeDistributionPackages
    /// - `fields_app_clip_default_experiences`: the fields to include for
    ///   returned resources of type appClipDefaultExperiences
    /// - `fields_app_store_review_details`: the fields to include for returned
    ///   resources of type appStoreReviewDetails
    /// - `fields_app_store_version_experiments`: the fields to include for
    ///   returned resources of type appStoreVersionExperiments
    /// - `fields_app_store_version_localizations`: the fields to include for
    ///   returned resources of type appStoreVersionLocalizations
    /// - `fields_app_store_version_phased_releases`: the fields to include for
    ///   returned resources of type appStoreVersionPhasedReleases
    /// - `fields_app_store_version_submissions`: the fields to include for
    ///   returned resources of type appStoreVersionSubmissions
    /// - `fields_app_store_versions`: the fields to include for returned
    ///   resources of type appStoreVersions
    /// - `fields_apps`: the fields to include for returned resources of type
    ///   apps
    /// - `fields_builds`: the fields to include for returned resources of type
    ///   builds
    /// - `fields_game_center_app_versions`: the fields to include for returned
    ///   resources of type gameCenterAppVersions
    /// - `fields_routing_app_coverages`: the fields to include for returned
    ///   resources of type routingAppCoverages
    /// - `filter_app_store_state`: filter by attribute 'appStoreState'
    /// - `filter_app_version_state`: filter by attribute 'appVersionState'
    /// - `filter_id`: filter by id(s)
    /// - `filter_platform`: filter by attribute 'platform'
    /// - `filter_version_string`: filter by attribute 'versionString'
    /// - `include`: comma-separated list of relationships to include
    /// - `limit`: maximum resources per page
    /// - `limit_app_store_version_experiments_v2`: maximum number of related
    ///   appStoreVersionExperimentsV2 returned (when they are included)
    /// - `limit_app_store_version_experiments`: maximum number of related
    ///   appStoreVersionExperiments returned (when they are included)
    /// - `limit_app_store_version_localizations`: maximum number of related
    ///   appStoreVersionLocalizations returned (when they are included)
    pub async fn apps_app_store_versions_get_to_many_related<'a>(
        &'a self,
        id: &'a str,
        fields_alternative_distribution_packages : Option < & 'a :: std :: vec :: Vec < types :: AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem > >,
        fields_app_clip_default_experiences: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem,
            >,
        >,
        fields_app_store_review_details: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem,
            >,
        >,
        fields_app_store_version_experiments: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem,
            >,
        >,
        fields_app_store_version_localizations: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem,
            >,
        >,
        fields_app_store_version_phased_releases: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem,
            >,
        >,
        fields_app_store_version_submissions: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem,
            >,
        >,
        fields_app_store_versions: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem,
            >,
        >,
        fields_apps: Option<
            &'a ::std::vec::Vec<types::AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem>,
        >,
        fields_builds: Option<
            &'a ::std::vec::Vec<types::AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem>,
        >,
        fields_game_center_app_versions: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem,
            >,
        >,
        fields_routing_app_coverages: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem,
            >,
        >,
        filter_app_store_state: Option<
            &'a ::std::vec::Vec<types::AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem>,
        >,
        filter_app_version_state: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem,
            >,
        >,
        filter_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        filter_platform: Option<
            &'a ::std::vec::Vec<types::AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem>,
        >,
        filter_version_string: Option<&'a ::std::vec::Vec<::std::string::String>>,
        include: Option<
            &'a ::std::vec::Vec<types::AppsAppStoreVersionsGetToManyRelatedIncludeItem>,
        >,
        limit: Option<i64>,
        limit_app_store_version_experiments_v2: Option<i64>,
        limit_app_store_version_experiments: Option<i64>,
        limit_app_store_version_localizations: Option<i64>,
    ) -> Result<ResponseValue<types::AppStoreVersionsResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/v1/apps/{}/appStoreVersions",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "fields[alternativeDistributionPackages]",
                &fields_alternative_distribution_packages,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appClipDefaultExperiences]",
                &fields_app_clip_default_experiences,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreReviewDetails]",
                &fields_app_store_review_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionExperiments]",
                &fields_app_store_version_experiments,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionLocalizations]",
                &fields_app_store_version_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionPhasedReleases]",
                &fields_app_store_version_phased_releases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionSubmissions]",
                &fields_app_store_version_submissions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersions]",
                &fields_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[apps]",
                &fields_apps,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[builds]",
                &fields_builds,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[gameCenterAppVersions]",
                &fields_game_center_app_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[routingAppCoverages]",
                &fields_routing_app_coverages,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[appStoreState]",
                &filter_app_store_state,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[appVersionState]",
                &filter_app_version_state,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[id]",
                &filter_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[platform]",
                &filter_platform,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[versionString]",
                &filter_version_string,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersionExperimentsV2]",
                &limit_app_store_version_experiments_v2,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersionExperiments]",
                &limit_app_store_version_experiments,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersionLocalizations]",
                &limit_app_store_version_localizations,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "apps_app_store_versions_get_to_many_related",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
