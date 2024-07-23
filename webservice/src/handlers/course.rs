use crate::dbaccess::course::*;
use crate::error::*;
use crate::models::course::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn post_new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<CreateCourse>,
) -> Result<HttpResponse, MyError> {
    println!("Received new course");
    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_coures_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id =
        i32::try_from(params.into_inner()).expect("get teacher_id from path param fail");
    get_coures_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    // let teacher_id = i32::try_from(teacher_id).expect("get teacher_id from path param fail");
    // let course_id = i32::try_from(course_id).expect("get course_id from path param fail");
    get_course_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
    update_course: web::Json<UpdateCourse>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    update_course_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[ignore]
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
        let course = web::Json(CreateCourse {
            teacher_id: 1,
            name: "Test Course".into(),
            description: Some("this is a course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: None,
            level: None,
        });

        let resp = post_new_course(app_state, course).await.unwrap();

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
        let teacher_id = web::Path::from(1);

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

        let params = web::Path::from((1, 1));

        let resp = get_course_detail(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn test_get_course_detail_fail() {
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

        let params = web::Path::from((1, 100));

        let resp = get_course_detail(app_state, params).await;

        match resp {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
    #[actix_rt::test]
    async fn update_test() {
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
        let course = web::Json(UpdateCourse {
            name: Some("new Test Course".into()),
            description: Some("this is another course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("spanish".into()),
            level: None,
        });

        let params = web::Path::from((5, 5));
        let resp = update_course_detail(app_state, params, course)
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn delete_get_course_success() {
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

        let params = web::Path::from((3, 3));

        let resp = delete_course(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn delete_get_course_fail() {
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

        let params = web::Path::from((3, 101));

        let resp = delete_course(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }
}
