// Mobile navigation toggle functionality
document.addEventListener('DOMContentLoaded', function() {
    const hamburger = document.querySelector('.hamburger');
    const navMenu = document.querySelector('.nav-menu');
    const dropdowns = document.querySelectorAll('.dropdown');

    // Toggle mobile menu
    if (hamburger) {
        hamburger.addEventListener('click', function() {
            const isExpanded = this.getAttribute('aria-expanded') === 'true';
            this.setAttribute('aria-expanded', !isExpanded);
            this.classList.toggle('active');
            navMenu.classList.toggle('active');
        });
    }

    // Close menu when clicking outside
    document.addEventListener('click', function(event) {
        const isClickInsideNav = event.target.closest('nav');
        const isMenuOpen = navMenu && navMenu.classList.contains('active');

        if (!isClickInsideNav && isMenuOpen) {
            hamburger.classList.remove('active');
            hamburger.setAttribute('aria-expanded', 'false');
            navMenu.classList.remove('active');
            closeAllDropdowns();
        }
    });

    function closeAllDropdowns() {
        dropdowns.forEach(function(dropdown) {
            dropdown.classList.remove('active');
            const toggle = dropdown.querySelector('.dropdown-toggle');
            if (toggle) toggle.setAttribute('aria-expanded', 'false');
        });
    }

    // Handle dropdown on mobile (click to toggle instead of hover)
    dropdowns.forEach(function(dropdown) {
        const toggle = dropdown.querySelector('.dropdown-toggle');
        if (!toggle) return;

        toggle.addEventListener('click', function(e) {
            if (window.innerWidth > 768) return;
            e.preventDefault();
            const isActive = dropdown.classList.contains('active');
            // Close all others first
            closeAllDropdowns();
            if (!isActive) {
                dropdown.classList.add('active');
                toggle.setAttribute('aria-expanded', 'true');
            }
        });
    });

    // Close mobile menu if window is resized to desktop
    window.addEventListener('resize', function() {
        if (window.innerWidth > 768) {
            if (navMenu && navMenu.classList.contains('active')) {
                hamburger.classList.remove('active');
                hamburger.setAttribute('aria-expanded', 'false');
                navMenu.classList.remove('active');
            }
            closeAllDropdowns();
        }
    });
});
