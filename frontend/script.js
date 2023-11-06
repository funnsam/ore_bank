let active_page = "login";

function switch_page(page) {
	let current = document.getElementById(`page_${active_page}`);
	let next = document.getElementById(`page_${page}`);
	current.classList.remove("active");
	next.classList.add("active");
}

window.addEventHandler("DOMContentLoaded", () => {
	document.getElementById("page_login_submit").onClick = () => {
		switch_page("home");
	};
});
