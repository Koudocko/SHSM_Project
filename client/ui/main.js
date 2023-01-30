function create_account(){
    const { invoke } = window.__TAURI__.tauri 

    let _username = document.getElementById("signup_username_input").value;
    let _password = document.getElementById("signup_password_input").value;
    let _password_confirm = document.getElementById("password_confirm_input").value;
    let _courseCode = document.getElementById("course_code_input").value;

    if (document.getElementById("role_select").selectedIndex == 0){
       var _isTeacher = true; 
    }
    else{
       var _isTeacher = false; 
    }

    invoke('create_account', { username: _username, password: [_password, _password_confirm], courseCode: _courseCode, isTeacher: _isTeacher })
			.then(() =>{});
}

function login_account(){
    const { invoke } = window.__TAURI__.tauri 

    let _username = document.getElementById("login_username_input").value;
    let _password = document.getElementById("login_password_input").value;

    invoke('login_account', { username: _username, password: _password })
			.then(() =>{});
}
