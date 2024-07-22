use crate::error::*;
use crate::models::course::Course;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

pub async fn get_coures_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows = sqlx::query!(
        r#"select id, teacher_id, name, time 
    from course
    where teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    let courses: Vec<Course> = rows
        .iter()
        .map(|r| Course {
            id: Some(r.id as usize),
            teacher_id: r.teacher_id as usize,
            name: r.name.clone(),
            time: Some(NaiveDateTime::from(r.time.expect("convert db time fail"))),
        })
        .collect();

    match courses.len() {
        0 => Err(MyError::NotFound(
            "Coures not found for teacher".to_string(),
        )),
        _ => Ok(courses),
    }
}

pub async fn get_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query!(
        r#"select id, teacher_id, name, time
        from course
        where teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_one(pool)
    .await;
    if let Ok(r) = row {
        Ok(Course {
            id: Some(r.id as usize),
            teacher_id: r.teacher_id as usize,
            name: r.name.clone(),
            time: Some(NaiveDateTime::from(r.time.expect("convert db time fail"))),
        })
    } else {
        Err(MyError::DBError("Course Id not found".into()))
    }
}
pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Result<Course, MyError> {
    let row = sqlx::query!(
        r#"insert into course (id, teacher_id, name)
    values($1,$2,$3)
    returning id, teacher_id, name, time"#,
        new_course.id.unwrap() as i32,
        new_course.teacher_id as i32,
        new_course.name
    )
    .fetch_one(pool)
    .await?;

    Ok(Course {
        id: Some(row.id as usize),
        teacher_id: row.teacher_id as usize,
        name: row.name.clone(),
        time: Some(NaiveDateTime::from(row.time.expect("convert db time fail"))),
    })
}
