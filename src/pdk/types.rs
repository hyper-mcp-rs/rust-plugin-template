#![allow(unused)]
use base64::engine::general_purpose::STANDARD;
use base64_serde::base64_serde_type;
use extism_pdk::{FromBytes, Json, ToBytes};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use std::collections::HashMap;

base64_serde_type!(Base64Standard, STANDARD);

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct Annotations {
    /// Intended audience for the resource
    pub audience: Vec<Role>,

    /// Last modified timestamp for the resource
    #[serde(rename = "lastModified")]
    pub last_modified: chrono::DateTime<chrono::Utc>,

    /// Priority level indicating the importance of the resource
    pub priority: f32,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct AudioContent {
    /// Optional additional metadata about the content block
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub meta: Option<Meta>,

    /// Optional content annotations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub annotations: Option<Annotations>,

    /// Base64-encoded audio data
    pub data: String,

    /// MIME type of the audio (e.g. 'audio/mpeg')
    #[serde(rename = "mimeType")]
    pub mime_type: String,

    pub r#type: AudioType,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum AudioType {
    #[default]
    #[serde(rename = "audio")]
    Audio,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct BlobResourceContents {
    /// Optional additional metadata about the blob resource
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub meta: Option<Meta>,

    /// Base64-encoded binary data of the resource
    pub blob: String,

    /// MIME type of the binary content (e.g. 'application/pdf')
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mime_type: Option<String>,

    /// URI of the resource
    pub uri: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct BooleanSchema {
    /// Optional default value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub default: Option<bool>,

    /// Description of the boolean input
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Optional human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,

    pub r#type: BooleanType,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum BooleanType {
    #[default]
    #[serde(rename = "boolean")]
    Boolean,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct CallToolRequest {
    pub context: PluginRequestContext,

    pub request: CallToolRequestParam,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct CallToolRequestParam {
    /// Arguments to pass to the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub arguments: Option<Map<String, Value>>,

    /// The name of the tool to call
    pub name: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct CallToolResult {
    /// Optional additional metadata about the tool call result
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub meta: Option<Meta>,

    /// Array of TextContent, ImageContent, AudioContent, EmbeddedResource, or ResourceLinks representing the result
    pub content: Vec<ContentBlock>,

    /// Whether the tool call ended in an error. If not set, defaults to false.
    #[serde(rename = "isError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_error: Option<bool>,

    /// Optional structured JSON result from the tool
    #[serde(rename = "structuredContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub structured_content: Option<Map<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct CompleteRequest {
    pub context: PluginRequestContext,

    pub request: CompleteRequestParam,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct CompleteRequestParam {
    pub argument: CompleteRequestParamArgument,

    /// Optional completion context with previously-resolved arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<CompleteRequestParamContext>,

    /// Reference to either a PromptReference or ResourceTemplateReference
    pub r#ref: Reference,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct CompleteRequestParamArgument {
    /// Name of the argument
    pub name: String,

    /// Current value to complete
    pub value: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct CompleteRequestParamContext {
    /// Previously-resolved argument values
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub arguments: Option<HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct CompleteResult {
    pub completion: CompleteResultCompletion,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct CompleteResultCompletion {
    /// Whether there are more completions available
    #[serde(rename = "hasMore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub has_more: Option<bool>,

    /// Total number of available completions
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub total: Option<i64>,

    /// Array of completion values (max 100 items)
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
#[serde(untagged)]
pub enum ContentBlock {
    Audio(AudioContent),
    EmbeddedResource(EmbeddedResource),
    Image(ImageContent),
    ResourceLink(ResourceLink),
    Text(TextContent),
    Empty(Empty),
}

impl Default for ContentBlock {
    fn default() -> Self {
        ContentBlock::Empty(Empty::default())
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct CreateMessageRequestParam {
    #[serde(rename = "includeContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_context: Option<CreateMessageRequestParamIncludeContext>,

    /// Maximum tokens to sample
    #[serde(rename = "maxTokens")]
    pub max_tokens: i64,

    /// Conversation messages of of TextContent, ImageContent or AudioContent
    pub messages: Vec<SamplingMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub metadata: Option<Value>,

    /// Preferences for model selection
    #[serde(rename = "modelPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub model_preferences: Option<ModelPreferences>,

    /// Stop sequences
    #[serde(rename = "stopSequences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stop_sequences: Option<Vec<String>>,

    /// Optional system prompt
    #[serde(rename = "systemPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub system_prompt: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub task: Option<Map<String, Value>>,

    /// Sampling temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub temperature: Option<f64>,

    #[serde(rename = "toolChoice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tool_choice: Option<ToolChoice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tools: Option<Vec<Tool>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum CreateMessageRequestParamIncludeContext {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "thisServer")]
    ThisServer,
    #[serde(rename = "allServers")]
    AllServers,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct CreateMessageResult {
    /// One of TextContent, ImageContent or AudioContent
    pub content: CreateMessageResultContent,

    /// Name of the model used
    pub model: String,

    pub role: Role,

    /// Optional reason sampling stopped
    #[serde(rename = "stopReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stop_reason: Option<String>,
}

type CreateMessageResultContent = SamplingMessage;

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ElicitationResponseNotificationParam {
    /// URI of the updated resource
    #[serde(rename = "elicitationId")]
    pub elicitation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
#[serde(tag = "mode")]
pub enum ElicitationRequestParam {
    #[serde(rename = "form")]
    Form {
        /// Message to present to the user
        #[serde(rename = "message")]
        message: String,

        #[serde(rename = "requestedSchema")]
        requested_schema: Schema,
    },
    #[serde(rename = "url")]
    Url {
        #[serde(rename = "elicitationId")]
        elicitation_id: String,

        /// Message to present to the user
        #[serde(rename = "message")]
        message: String,

        #[serde(rename = "url")]
        url: String,
    },
}

impl Default for ElicitationRequestParam {
    fn default() -> Self {
        ElicitationRequestParam::Form {
            message: String::new(),
            requested_schema: Schema::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ElicitationRequestParamWithTimeout {
    #[serde(flatten)]
    pub inner: ElicitationRequestParam,

    /// Optional timeout in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub timeout: Option<i64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ElicitationResult {
    pub action: ElicitationResultAction,

    /// Form data submitted by user (only present when action is accept)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content: Option<HashMap<String, ElicitationResultContentValue>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum ElicitationResultAction {
    #[default]
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "decline")]
    Decline,
    #[serde(rename = "cancel")]
    Cancel,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
#[serde(untagged)]
pub enum ElicitationResultContentValue {
    String(String),
    Number(Number), // or serde_json::Number if you want exactness
    Bool(bool),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct EmbeddedResource {
    /// Optional additional metadata about the embedded resource
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub meta: Option<Meta>,

    /// Optional resource annotations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub annotations: Option<Annotations>,

    /// The embedded TextResourceContents or BlobResourceContents
    pub resource: ResourceContents,

    pub r#type: ResourceType,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Empty {}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct EnumSchema {
    /// Description of the enum input
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Array of allowed string values
    pub r#enum: Vec<String>,

    /// Optional array of human-readable names for the enum values
    #[serde(rename = "enumNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enum_names: Option<Vec<String>>,

    /// Optional human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,

    pub r#type: StringType,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct GetPromptRequest {
    pub context: PluginRequestContext,

    pub request: GetPromptRequestParam,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct GetPromptRequestParam {
    /// Arguments for templating the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub arguments: Option<HashMap<String, String>>,

    /// Name of the prompt to retrieve
    pub name: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct GetPromptResult {
    /// Optional description of the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Array of prompt messages
    pub messages: Vec<PromptMessage>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ImageContent {
    /// Optional additional metadata about the content block
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub meta: Option<Meta>,

    /// Optional content annotations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub annotations: Option<Annotations>,

    /// Base64-encoded image data
    pub data: String,

    /// MIME type of the image (e.g. 'image/png')
    #[serde(rename = "mimeType")]
    pub mime_type: String,

    pub r#type: ImageType,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum ImageType {
    #[default]
    #[serde(rename = "image")]
    Image,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct KeyringEntryId {
    pub service: String,
    pub user: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ListPromptsRequest {
    pub context: PluginRequestContext,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ListPromptsResult {
    /// Array of available prompts
    pub prompts: Vec<Prompt>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ListResourcesRequest {
    pub context: PluginRequestContext,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ListResourcesResult {
    /// Array of available resources
    pub resources: Vec<Resource>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ListResourceTemplatesRequest {
    pub context: PluginRequestContext,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ListResourceTemplatesResult {
    /// Array of resource templates
    #[serde(rename = "resourceTemplates")]
    pub resource_templates: Vec<ResourceTemplate>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ListRootsResult {
    /// Array of root directories/resources
    pub roots: Vec<Root>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ListToolsRequest {
    pub context: PluginRequestContext,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ListToolsResult {
    /// Array of available tools
    pub tools: Vec<Tool>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum LoggingLevel {
    #[default]
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "emergency")]
    Emergency,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct LoggingMessageNotificationParam {
    /// Data to log (any JSON-serializable type)
    pub data: Value,

    pub level: LoggingLevel,

    /// Optional logger name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub logger: Option<String>,
}

type Meta = Map<String, Value>;

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ModelHint {
    /// Suggested model name or family
    pub name: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ModelPreferences {
    /// Priority for cost (0-1)
    pub cost_priority: f32,

    /// Model name hints
    pub hints: Vec<ModelHint>,

    /// Priority for intelligence (0-1)
    #[serde(rename = "intelligencePriority")]
    pub intelligence_priority: f32,

    /// Priority for speed (0-1)
    #[serde(rename = "speedPriority")]
    pub speed_priority: f32,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct NumberSchema {
    /// Description of the number input
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Maximum value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub maximum: Option<f64>,

    /// Minimum value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub minimum: Option<f64>,

    /// Optional human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,

    pub r#type: NumberType,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum NumberType {
    #[default]
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "integer")]
    Integer,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum ObjectType {
    #[default]
    #[serde(rename = "object")]
    Object,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct PluginNotificationContext {
    /// Additional metadata about the notification
    #[serde(rename = "_meta")]
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct PluginRequestContext {
    /// Additional metadata about the request
    #[serde(rename = "_meta")]
    pub meta: Meta,

    /// Unique identifier for this request
    pub id: PluginRequestId,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
#[serde(untagged)]
pub enum PluginRequestId {
    String(String),
    Number(i64),
}

impl Default for PluginRequestId {
    fn default() -> Self {
        PluginRequestId::String(String::new())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
#[serde(untagged)]
pub enum PrimitiveSchemaDefinition {
    Boolean(BooleanSchema),
    Enum(EnumSchema),
    Number(NumberSchema),
    String(StringSchema),
    Empty(Empty),
}

impl Default for PrimitiveSchemaDefinition {
    fn default() -> Self {
        PrimitiveSchemaDefinition::Empty(Empty::default())
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ProgressNotificationParam {
    /// Optional progress message describing current operation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message: Option<String>,

    /// The progress thus far
    pub progress: f64,

    /// A token identifying the progress context
    #[serde(rename = "progressToken")]
    pub progress_token: String,

    /// Optional total units of work
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub total: Option<f64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct Prompt {
    /// Optional prompt arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub arguments: Option<Vec<PromptArgument>>,

    /// Description of what the prompt does
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Unique name of the prompt
    pub name: String,

    /// Human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct PromptArgument {
    /// Description of the argument
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Name of the argument
    pub name: String,

    /// Whether this argument is required
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub required: Option<bool>,

    /// Human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct PromptMessage {
    /// One of TextContent, ImageContent, AudioContent, EmbeddedResource, or ResourceLink
    pub content: ContentBlock,

    pub role: Role,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct PromptReference {
    /// Name of the prompt
    pub name: String,

    /// Optional human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,

    pub r#type: PromptReferenceType,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum PromptReferenceType {
    #[default]
    #[serde(rename = "prompt")]
    Prompt,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ReadResourceRequest {
    pub context: PluginRequestContext,

    pub request: ReadResourceRequestParam,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ReadResourceRequestParam {
    /// URI of the resource to read
    pub uri: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ReadResourceResult {
    /// Array of TextResourceContents or BlobResourceContents
    pub contents: Vec<ResourceContents>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
#[serde(untagged)]
pub enum Reference {
    Prompt(PromptReference),
    ResourceTemplate(ResourceTemplateReference),
    Empty(Empty),
}

impl Default for Reference {
    fn default() -> Self {
        Reference::Empty(Empty::default())
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct Resource {
    /// Optional resource annotations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub annotations: Option<Annotations>,

    /// Description of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// MIME type of the resource
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mime_type: Option<String>,

    /// Human-readable name
    pub name: String,

    /// Size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub size: Option<i64>,

    /// Human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,

    /// URI of the resource
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
#[serde(untagged)]
pub enum ResourceContents {
    Blob(BlobResourceContents),
    Text(TextResourceContents),
    Empty(Empty),
}

impl Default for ResourceContents {
    fn default() -> Self {
        ResourceContents::Empty(Empty::default())
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ResourceLink {
    /// Optional additional metadata about the resource link
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub meta: Option<Meta>,

    /// Optional resource annotations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub annotations: Option<Annotations>,

    /// Optional description of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Optional MIME type of the resource
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mime_type: Option<String>,

    /// Optional human-readable name
    pub name: String,

    /// Optional size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub size: Option<i64>,

    /// Optional human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,

    pub r#type: ResourceLinkType,

    /// URI of the resource
    pub uri: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum ResourceLinkType {
    #[default]
    #[serde(rename = "resource_link")]
    ResourceLink,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum ResourceReferenceType {
    #[default]
    #[serde(rename = "resource")]
    Resource,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ResourceTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub annotations: Option<Annotations>,

    /// Description of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// MIME type for resources matching this template
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mime_type: Option<String>,

    /// Human-readable name
    pub name: String,

    /// Human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,

    /// RFC 6570 URI template pattern
    #[serde(rename = "uriTemplate")]
    pub uri_template: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ResourceTemplateReference {
    pub r#type: ResourceReferenceType,

    /// URI or URI template pattern of the resource
    pub uri: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum ResourceType {
    #[default]
    #[serde(rename = "resource")]
    Resource,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ResourceUpdatedNotificationParam {
    /// URI of the updated resource
    pub uri: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum Role {
    #[default]
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct Root {
    /// Optional human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,

    /// URI of the root (typically file://)
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
#[serde(untagged)]
pub enum SamplingMessage {
    Audio(AudioContent),
    Image(ImageContent),
    Text(TextContent),
    Empty(Empty),
}

impl Default for SamplingMessage {
    fn default() -> Self {
        SamplingMessage::Empty(Empty::default())
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct Schema {
    /// A map of StringSchema, NumberSchema, BooleanSchema or EnumSchema definitions (no nesting)
    pub properties: HashMap<String, PrimitiveSchemaDefinition>,

    /// Required property names
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub required: Option<Vec<String>>,

    pub r#type: ObjectType,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct StringSchema {
    /// Description of the string input
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub format: Option<StringSchemaFormat>,

    /// Maximum length of the string
    #[serde(rename = "maxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_length: Option<i64>,

    /// Minimum length of the string
    #[serde(rename = "minLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min_length: Option<i64>,

    /// Optional human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,

    pub r#type: StringType,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum StringSchemaFormat {
    #[default]
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "uri")]
    Uri,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "date_time")]
    Datetime,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum StringType {
    #[default]
    #[serde(rename = "string")]
    String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct TextContent {
    /// Optional additional metadata about the content block
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub meta: Option<Meta>,

    /// Optional content annotations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub annotations: Option<Annotations>,

    /// The text content
    pub text: String,

    pub r#type: TextType,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct TextResourceContents {
    /// Optional additional metadata about the text resource
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub meta: Option<Meta>,

    /// MIME type of the text content (e.g. 'text/plain')
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mime_type: Option<String>,

    /// Text content of the resource
    pub text: String,

    /// URI of the resource
    pub uri: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum TextType {
    #[default]
    #[serde(rename = "text")]
    Text,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct Tool {
    /// Optional tool annotations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub annotations: Option<Annotations>,

    /// Description of what the tool does
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    #[serde(rename = "inputSchema")]
    pub input_schema: ToolSchema,

    /// Unique name of the tool
    pub name: String,

    #[serde(rename = "outputSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub output_schema: Option<ToolSchema>,

    /// Human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ToolChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mode: Option<ToolChoiceMode>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub enum ToolChoiceMode {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "none")]
    None,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
pub struct ToolSchema {
    /// Schema properties
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub properties: Option<Map<String, Value>>,

    /// Required properties
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub required: Option<Vec<String>>,

    pub r#type: ObjectType,
}
