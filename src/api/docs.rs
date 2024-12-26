use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::login,
        auth::register,
        documents::create_document,
        documents::get_document,
        documents::update_document,
        documents::update_permissions,
        tasks::create_task,
        tasks::update_task,
        tasks::delete_task,
        users::list_users,
        users::get_user,
        users::update_user,
        users::delete_user,
    ),
    components(
        schemas(
            auth::LoginRequest,
            auth::RegisterRequest,
            documents::CreateDocumentRequest,
            documents::UpdateDocumentRequest,
            documents::UpdatePermissionRequest,
            tasks::CreateTaskRequest,
            tasks::UpdateTaskRequest,
            models::User,
            models::Document,
            models::DocumentPermission,
            models::ScheduledTask,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "documents", description = "Document management endpoints"),
        (name = "tasks", description = "Task management endpoints"),
        (name = "users", description = "User management endpoints"),
    )
)]
pub struct ApiDoc;

pub fn create_docs() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())
} 