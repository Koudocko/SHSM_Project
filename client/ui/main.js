function create_account(){
    const { invoke } = window.__TAURI__.tauri 

    let _username = document.getElementById("username_input").value;
    let _password = document.getElementById("password_input").value;
    let _courseCode = document.getElementById("course_code_input").value;

    if (document.getElementById("role_select").selectedIndex == 0){
       var _isTeacher = true; 
    }
    else{
       var _isTeacher = false; 
    }

    invoke('create_account', { username: _username, password: _password, courseCode: _courseCode, isTeacher: _isTeacher })
			.then(() =>{});
}
