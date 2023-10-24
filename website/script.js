function createBox(post) {
    let box = document.createElement("div");
    box.className = "box";
    let term = document.createElement("p");
    term.textContent = post.english;
    box.appendChild(term);
    let begreb = document.createElement("p");
    begreb.textContent = post.danish;
    box.appendChild(begreb);
    let kilde = document.createElement("p");
    kilde.textContent = post.source;
    box.appendChild(kilde);
    return box;
}

const inputSearch = document.querySelector("#search-field");
const boxesDiv = document.querySelector("#boxes")

let timer;

inputSearch.addEventListener("input", () => {
    boxesDiv.innerHTML = "";
    clearTimeout(timer);
    const ms = 70;
    timer = setTimeout(() => {
        let value = inputSearch.value;
        if (value.length >= 3) {
            fetch(`http://localhost:3030/`, {
                method: "POST",
                body: value
            }).then(res => res.json())
              .then(posts => {
                boxesDiv.innerHTML = "";
                if (posts.length == 0) {
                    let p = document.createElement("p");
                    p.textContent = "Intet resultat.";
                    boxesDiv.appendChild(p);
                }
                for (let i = 0; i < posts.length; i++) {
                    const post = posts[i];
                    box = createBox(post);     
                    boxesDiv.appendChild(box);               
                }
            });
        }
    }, ms);
});
