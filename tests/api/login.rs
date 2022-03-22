use crate::helpers::assert_is_redirect_to;
use crate::helpers::spawn_app;

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": "random-usernameee",
        "password": "random-password"
    });
    let response = app.post_login(&login_body).await;

    assert_is_redirect_to(&response, "/login");
    // Act - Part 2 - Follow the redirect
    let html_page = app.get_login_html().await;
    println!("{}", html_page);
    assert!(html_page.contains("<p><i>Authentication Failed</i></p>"));
    // Act - Part 3 - Reload the login page
    // let response = app.get_login().await;
    // let html_page = response.text().await.unwrap();
    // assert!(!html_page.contains(r#"<p><i>Authentication failed</i></p>"#));
}

#[tokio::test]
async fn redirect_to_admin_dashboard_after_login_success() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));
}
