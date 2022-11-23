
let sortColumn = 0
let sortAsc = 1;

function toggle(n) {
    const elements = document.querySelectorAll(`tr td:nth-child(${n + 1})`)
    const header = document.querySelector(`tr th:nth-child(${n + 1})`)
    header.classList.toggle('hidden')
    elements.forEach((elt) => {
        elt.classList.toggle('hidden')
    })
    const text = document.querySelector(`button:nth-child(${n + 1}) span`)
    if (text.innerHTML === 'Ocultar') {
        text.innerHTML = 'Mostrar'
    } else {
        text.innerHTML = 'Ocultar'
    }
}

function sortTable(n) {
    if (n === sortColumn) {
        sortAsc = -sortAsc;
        let arrow = '↓';
        if (sortAsc !== 1) {
            arrow = '↑';
        }
        document.querySelector(`th:nth-child(${n + 1}) span`).innerText = arrow;
    } else {
        document.querySelector(`th:nth-child(${sortColumn + 1}) span`).innerText = '';
        document.querySelector(`th:nth-child(${n + 1}) span`).innerText = '↓';
        sortColumn = n;
        sortAsc = 1;
    }

    const elements = document.querySelectorAll('tbody tr')
    const elementsSlice = [].slice.call(elements)
    const elementsWithInfo = elementsSlice.map((a) => {
        if (n === 1) {
            return [a, a.childNodes[1].firstChild.innerText]
        } else if (n === 0 || n === 3) {
            return [a, parseInt(a.childNodes[n].innerText)]
        } else {
            return [a, a.childNodes[n].innerText]
        }
    })
    elementsWithInfo.sort(([_a, contentsA], [_b, contentsB]) => {
        if (contentsA >= contentsB) {
            return sortAsc
        } else if (contentsA === contentsB) {
            return 0
        } else {
            return -sortAsc
        }
    })
    const body = document.querySelector('tbody')
    body.innerHTML = ''
    elementsWithInfo.forEach(([a, _]) => body.appendChild(a))
}
