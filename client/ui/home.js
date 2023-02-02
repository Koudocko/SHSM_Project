document.getElementById("posted-announcement-container").addEventListener("click", function(e){
    if (e.target.className == "delete"){
        const { invoke } = window.__TAURI__.tauri 
        var _title = e.target.parentNode.querySelector('.title').textContent;
    
        invoke('remove_announcement', { title: _title })
          .then(() =>{});
    }
});
