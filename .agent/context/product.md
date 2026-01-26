# Initial Concept

## Vision
To provide a simpler, easier-to-extend, and more featureful successor to CSL (Citation Style Language). The project aims to modernize citation processing with a Rust-based model that generates JSON schemas, ensuring alignment between code and configuration while offering high performance for both batch and interactive contexts.

## Target Audience
- **Software Developers:** Developers building bibliographic tools (like Zotero, Pandoc, or other reference managers) who require a robust, high-performance citation engine to handle complex formatting and data processing tasks.

## Core Features
- **High-Performance Processing:** Optimized for both batch processing (e.g., Markdown, LaTeX documents) and real-time interactive use (e.g., GUI reference managers), ensuring speed and efficiency.
- **Simplified Style Configuration:** Moves logic from complex templates to extensible option groups, making style creation and maintenance easier for users and developers.
- **Modern Standards:** Native support for EDTF (Extended Date/Time Format) and other modern idioms, replacing legacy string parsing with structured data handling.
- **Schema-Driven Development:** JSON schemas are generated directly from the Rust model, ensuring consistency and providing a contract for external tools and domain experts.
- **Cross-Platform Compatibility:** Designed to work across desktop, web, and CLI environments.
