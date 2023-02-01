$(function () {
    $(document).scroll(function () {
        var $nav = $("#mainNavbar");
        $nav.toggleClass("scrolled", $(this).scrollTop() > $nav.height());
    });
});

const addButton = document.querySelector("#add-event-btn");
addButton.addEventListener('click', () => {
    console.log("HElLO");
    const modal = document.querySelector("#add-event-modal");
    modal.style.display = 'block';
});

// Add event listener to the close buttons in the modals
const closeButtons = document.querySelectorAll('.close-btn');
closeButtons.forEach(button => {
    button.addEventListener('click', () => {
        const modal = button.parentNode.parentNode;
        modal.style.display = 'none';
    });
});