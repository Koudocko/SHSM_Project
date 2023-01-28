function create_account(){
    const { invoke } = window.__TAURI__.tauri 

    let _username = document.getElementById("username_input").value;
    let _password = document.getElementById("password_input").value;

    if (document.getElementById("password_input").selectedIndex == "0"){
       var _isTeacher = true; 
    }
    else{
       var _isTeacher = false; 
    }

    invoke('create_account', { username: _username, password: _password, isTeacher: _isTeacher })
			.then(() =>{});
}
