// Mobile navigation toggle functionality
document.addEventListener('DOMContentLoaded', function() {
    const hamburger = document.querySelector('.hamburger');
    const navMenu = document.querySelector('.nav-menu');
    const dropdownToggle = document.querySelector('.dropdown-toggle');
    const dropdown = document.querySelector('.dropdown');

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
        }
    });

    // Handle dropdown on mobile (click to toggle instead of hover)
    if (dropdownToggle && window.innerWidth <= 768) {
        dropdownToggle.addEventListener('click', function(e) {
            e.preventDefault();
            dropdown.classList.toggle('active');
        });
    }

    // Re-enable dropdown click behavior on resize
    window.addEventListener('resize', function() {
        if (window.innerWidth <= 768) {
            if (dropdownToggle && !dropdownToggle.hasEventListener) {
                dropdownToggle.hasEventListener = true;
                dropdownToggle.addEventListener('click', function(e) {
                    e.preventDefault();
                    dropdown.classList.toggle('active');
                });
            }
        } else {
            // Close mobile menu if window is resized to desktop
            if (navMenu && navMenu.classList.contains('active')) {
                hamburger.classList.remove('active');
                hamburger.setAttribute('aria-expanded', 'false');
                navMenu.classList.remove('active');
            }
            if (dropdown) {
                dropdown.classList.remove('active');
            }
        }
    });
});
