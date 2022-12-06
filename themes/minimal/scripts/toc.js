function generateTableOfContents() {
    const headings = Array.from(document.querySelectorAll('h2, h3, h4, h5, h6'))
    const tocParentNode = document.querySelector('[data-toc]')
    if (headings.length === 0) {
        tocParentNode.parentNode.removeChild(tocParentNode)
        return
    }
    for (const el of headings) {
        const elAnchorNode = el.querySelector('.anchor')
        const elNode = document.createElement('a')
        elNode.href = `#${elAnchorNode.id}`
        elNode.textContent = el.textContent
        elNode.dataset.depth = parseInt(el.tagName[el.tagName.length - 1]) - 1
        tocParentNode.appendChild(elNode)
    }
}

document.addEventListener('DOMContentLoaded', () => {
    if (window.location.pathname.includes('posts/')) {
        generateTableOfContents();
    }
})