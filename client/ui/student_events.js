document.getElementsByClassName("event-container")[0].addEventListener("click", function(e){
    const { invoke } = window.__TAURI__.tauri 

    var _title = e.target.parentNode.querySelector(".event-title").textContent;

    if (e.target.className == "event-signup"){
        invoke('modify_user_event', { title: _title })
          .then(() =>{});
    }
})
