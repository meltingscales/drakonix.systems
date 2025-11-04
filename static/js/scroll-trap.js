// Anti-infinite-scroll auto-scroll-up behavior
// Only runs on homepage
if (window.location.pathname === '/') {
    // Generate checkered pattern for scroll-pattern elements
    function generatePattern(element) {
        const word = element.getAttribute('data-word');
        if (!word) return;

        const sep = '        '; // 8 spaces between words
        const offset = '      '; // 6 spaces offset on alternating rows

        // Calculate dimensions based on viewport
        const charWidth = 10; // approximate char width in pixels at 1.2rem
        const lineHeight = 1.5 * 19.2; // line-height * font-size in pixels
        const width = Math.floor(window.innerWidth / charWidth);

        // Calculate height to fill 500vh (5 pages)
        const scrollTrapSection = element.closest('.scroll-trap');
        const scrollTrapHeight = scrollTrapSection ? scrollTrapSection.offsetHeight : window.innerHeight * 5;
        const height = Math.ceil(scrollTrapHeight / lineHeight) + 10; // Add extra rows for safety

        let lines = [];
        for (let row = 0; row < height; row++) {
            let line = '';
            if (row % 2 === 1) {
                line += offset;
            }
            // Fill line with repeating pattern
            while (line.length < width + word.length + sep.length) {
                line += word;
                line += sep;
            }
            // Trim to visible width
            line = line.substring(0, width);
            lines.push(line);
        }

        element.textContent = lines.join('\n');
    }

    // Generate patterns for all scroll-pattern elements
    const patterns = document.querySelectorAll('.scroll-pattern');
    patterns.forEach(generatePattern);

    // Regenerate on resize
    let resizeTimeout;
    window.addEventListener('resize', function() {
        clearTimeout(resizeTimeout);
        resizeTimeout = setTimeout(function() {
            patterns.forEach(generatePattern);
        }, 250);
    });

    let hasEnteredScrollTrap = false;
    let scrollingBack = false;
    let isPaused = false;
    let userScrolledDuringAutoscroll = false;
    const SCROLL_SPEED = 2; // pixels per frame
    const SCROLL_THRESHOLD = 100; // px from top of first scroll trap to trigger
    const PAUSE_DURATION = 5000; // 5 seconds pause when user scrolls

    // Get the first scroll trap section
    const firstScrollTrap = document.getElementById('scroll-trap-stop');

    if (firstScrollTrap) {
        // Detect user-initiated scroll vs programmatic scroll
        let lastScrollTime = 0;
        let programmaticScroll = false;

        // Check if user has scrolled past the threshold
        function checkScrollPosition() {
            if (scrollingBack || isPaused) return;

            const rect = firstScrollTrap.getBoundingClientRect();
            const scrolledIntoTrap = rect.top < window.innerHeight - SCROLL_THRESHOLD;

            if (scrolledIntoTrap && !hasEnteredScrollTrap) {
                hasEnteredScrollTrap = true;
                startAutoScrollUp();
            }
        }

        // Pause auto-scroll when user tries to scroll
        function pauseAutoScroll() {
            if (!scrollingBack) return;

            isPaused = true;
            userScrolledDuringAutoscroll = true;

            setTimeout(function() {
                isPaused = false;
                if (scrollingBack) {
                    // Resume scrolling
                    requestAnimationFrame(scrollStep);
                }
            }, PAUSE_DURATION);
        }

        // Smooth auto-scroll back up
        function startAutoScrollUp() {
            scrollingBack = true;
            userScrolledDuringAutoscroll = false;
            requestAnimationFrame(scrollStep);
        }

        function scrollStep() {
            if (isPaused) return;

            const currentScroll = window.pageYOffset || document.documentElement.scrollTop;

            // Calculate target position (just above the scroll trap)
            const targetPosition = firstScrollTrap.offsetTop - window.innerHeight;

            if (currentScroll > targetPosition && scrollingBack) {
                programmaticScroll = true;
                window.scrollBy(0, -SCROLL_SPEED);
                programmaticScroll = false;
                requestAnimationFrame(scrollStep);
            } else {
                // Finished scrolling back
                scrollingBack = false;
                hasEnteredScrollTrap = false;
                isPaused = false;
            }
        }

        // Listen for scroll events with throttling
        let scrollTimeout;
        let lastScrollPosition = 0;

        window.addEventListener('scroll', function() {
            // Detect user scroll (not programmatic)
            const currentScrollPosition = window.pageYOffset || document.documentElement.scrollTop;
            const now = Date.now();

            if (!programmaticScroll && scrollingBack && !isPaused) {
                // User is trying to scroll while auto-scroll is active
                if (Math.abs(currentScrollPosition - lastScrollPosition) > SCROLL_SPEED * 2) {
                    pauseAutoScroll();
                }
            }

            lastScrollPosition = currentScrollPosition;
            lastScrollTime = now;

            if (scrollTimeout) {
                clearTimeout(scrollTimeout);
            }
            scrollTimeout = setTimeout(checkScrollPosition, 50);
        }, { passive: true });

        // Also detect wheel events for more immediate response
        window.addEventListener('wheel', function(e) {
            if (scrollingBack && !isPaused) {
                pauseAutoScroll();
            }
        }, { passive: true });

        // Detect touch events on mobile
        window.addEventListener('touchmove', function(e) {
            if (scrollingBack && !isPaused) {
                pauseAutoScroll();
            }
        }, { passive: true });
    }
}
