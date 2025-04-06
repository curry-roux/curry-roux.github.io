import("../pkg/index.js").catch(console.error);

// SPAルーターの設定
const routes = {
    "/": "home",
    "/about": "About content",
    "/test": "Test content",
}

function navigate(path) {
    // window.history.pushState({}, path, window.location.origin + path);
    window.history.pushState({ path }, "", path);
    render(path);   
}

function render(path) {
    console.log(path);
    // wasmの関数を呼び出す
    import("../pkg/index.js").then(module => {
        if (path === "/") module.test1();
        if (path === "/about") module.test2();
    }).catch(console.error);
}

// Intercept link clicks
document.addEventListener("click", (e) => {
    const target = e.target.closest("a");
    if (target && target.matches("[data-route]")) {
        e.preventDefault();
        navigate(target.getAttribute("href"));
    }
    });

    // Handle browser navigation events (back/forward)
    window.addEventListener("popstate", (e) => {
        const path = e.state?.path || "/";
        //navigate(window.location.pathname);
        navigate(path);
    });

    // Initialize on page load
    navigate(window.location.pathname);