use crate::errors::MyError;
use crate::models::{TeacherRegisterForm, TeacherResponse};
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;

pub async fn get_all_teachers(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    // 相当于一个http客户端
    let awc_client = awc::Client::default();
    let res = awc_client
        .get("http://localhost:3000/teachers/")
        .send()
        .await
        .unwrap()
        .json::<Vec<TeacherResponse>>() // 将teacherlist转换成这种类型
        .await
        .unwrap();

    // 用context可以向模板里添加数据
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("teacher", &res);

    // 渲染模板
    let s = tmpl
        .render("teachers.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");
    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<TeacherRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    if params.name == "Dave" {
        ctx.insert("error", "Dave already exists!");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| MyError::TeraError("Template error".to_string()))
            .unwrap();
    } else {
        let new_teacher = json!({
            "name":&params.name,
            "picture_url": &params.imageurl,
            "profile":&params.profile,
        });
        let awc_client = awc::Client::default();
        let res = awc_client
            .post("http://127.0.0.1:3000/teachers/")
            .send_json(&new_teacher)
            .await
            .unwrap()
            .body()
            .await?;

        let teacher_response: TeacherResponse = serde_json::from_str(std::str::from_utf8(&res)?)?;

        s = format!("Congratulations! Your Id is: {}.", teacher_response.id);
    }

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
