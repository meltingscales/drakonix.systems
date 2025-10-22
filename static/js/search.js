// Client-side search functionality
(function() {
    let searchIndex = [];
    let searchInput = document.getElementById('search');

    if (!searchInput) return;

    // Fetch search index on page load
    fetch('/search.json')
        .then(response => response.json())
        .then(data => {
            searchIndex = data;
        })
        .catch(err => console.error('Failed to load search index:', err));

    // Create search results container
    const resultsContainer = document.createElement('div');
    resultsContainer.id = 'search-results';
    resultsContainer.style.display = 'none';
    searchInput.parentElement.style.position = 'relative';
    searchInput.parentElement.appendChild(resultsContainer);

    // Search function
    function performSearch(query) {
        if (!query || query.length < 2) {
            resultsContainer.style.display = 'none';
            return;
        }

        const lowerQuery = query.toLowerCase();
        const results = searchIndex.filter(entry => {
            return entry.title.toLowerCase().includes(lowerQuery) ||
                   entry.content.toLowerCase().includes(lowerQuery) ||
                   entry.tags.some(tag => tag.toLowerCase().includes(lowerQuery));
        }).slice(0, 10); // Limit to 10 results

        if (results.length === 0) {
            resultsContainer.innerHTML = '<ul><li>No results found</li></ul>';
            resultsContainer.style.display = 'block';
            return;
        }

        const resultsHTML = '<ul>' + results.map(entry => {
            const contentPreview = entry.content.substring(0, 100) + '...';
            return `
                <li>
                    <a href="${entry.url}">
                        <strong>${highlightText(entry.title, lowerQuery)}</strong>
                        <br>
                        <small>${highlightText(contentPreview, lowerQuery)}</small>
                    </a>
                </li>
            `;
        }).join('') + '</ul>';

        resultsContainer.innerHTML = resultsHTML;
        resultsContainer.style.display = 'block';
    }

    // Highlight matching text
    function highlightText(text, query) {
        const regex = new RegExp(`(${escapeRegex(query)})`, 'gi');
        return text.replace(regex, '<mark>$1</mark>');
    }

    // Escape regex special characters
    function escapeRegex(str) {
        return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    }

    // Event listeners
    searchInput.addEventListener('input', (e) => {
        performSearch(e.target.value);
    });

    searchInput.addEventListener('blur', () => {
        // Delay hiding to allow clicking on results
        setTimeout(() => {
            resultsContainer.style.display = 'none';
        }, 200);
    });

    searchInput.addEventListener('focus', (e) => {
        if (e.target.value.length >= 2) {
            performSearch(e.target.value);
        }
    });

    // Close search results when clicking outside
    document.addEventListener('click', (e) => {
        if (!searchInput.contains(e.target) && !resultsContainer.contains(e.target)) {
            resultsContainer.style.display = 'none';
        }
    });
})();
