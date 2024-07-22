use super::db_access::*;
use super::error::MyError;
use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<Course>,
) -> Result<HttpResponse, MyError> {
    println!("Received new course");
    post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_coures_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> Result<HttpResponse, MyError> {
    let teacher_id =
        i32::try_from(params.into_inner()).expect("get teacher_id from path param fail");
    get_coures_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    let teacher_id = i32::try_from(teacher_id).expect("get teacher_id from path param fail");
    let course_id = i32::try_from(course_id).expect("get course_id from path param fail");
    get_course_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_test() {
        dotenv().ok();
        let sql_url = env::var("DATABASE_URL").expect("can't find db url");
        let db_pool = PgPoolOptions::new()
            .connect(&sql_url)
            .await
            .expect("can't connect to db");
        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            // courses: Mutex::new(vec![]),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test Course".into(),
            id: Some(1),
            time: None,
        });

        let resp = new_course(app_state, course).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn test_get_coures_for_teacher_success() {
        dotenv().ok();
        let sql_url = env::var("DATABASE_URL").expect("can't find db url");
        let db_pool = PgPoolOptions::new()
            .connect(&sql_url)
            .await
            .expect("can't connect to db");
        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            // courses: Mutex::new(vec![]),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher_id = web::Path::from(1 as usize);

        let resp = get_coures_for_teacher(app_state, teacher_id).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn test_get_course_detail_success() {
        dotenv().ok();
        let sql_url = env::var("DATABASE_URL").expect("can't find db url");
        let db_pool = PgPoolOptions::new()
            .connect(&sql_url)
            .await
            .expect("can't connect to db");
        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            // courses: Mutex::new(vec![]),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params = web::Path::from((1 as usize, 1 as usize));

        let resp = get_course_detail(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }
}
