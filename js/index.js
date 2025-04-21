import("../pkg/index.js").catch(console.error);

// SPAルーターの設定
const routes = {
    "/": "home",
    "/about": "About content",
    "/test": "Test content",
}

function navigate(path) {
    window.history.pushState({ path }, "", path);
    render(path);   
}

function render(path) {
    console.log(path);
    document.body.classList.remove("body-locked");

    // wasmの関数を呼び出す
    import("../pkg/index.js").then(module => {
        if (path === "/") {
            reset();
            document.body.classList.add("body-locked");
            module.test1();
        }
        if (path === "/about") {
            reset();
            module.stop_loop();

            const canvas = document.getElementById("canvas");
            canvas.style.position = "absolute";

            path = "test.md";
            render_md(path);
        }
    }).catch(console.error);
}

async function render_md(path) {
    const res = await fetch(path);
    const markdown = await res.text();
    const html = marked.parse(markdown);

    console.log(html);
    
    let content = document.getElementById("content");
    try {
        content.innerHTML = html;
    } catch (e) {
        console.error("Error rendering markdown:", e);
        content.innerHTML = "<p>Error rendering markdown</p>";
    }
}

function reset(){
    let content = document.getElementById("content");
    html = null;
    content.innerHTML = html;
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