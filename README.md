### figma

:construction: work-in-progress

Collection of Rust-based tooling for interacting with Figma's API.

- [figma-api](./figma-api): Wrapper around Figma API + Types.
- [figma-to-react](./figma-to-react): Eventual transpilation from Figma to React Components.

ideas:

- Obtain document node tree
- Generate internal representation of components, base styling and layout.
- Optimize IR by matching recognized components, reductive refactoring?
- Use optimized AST to generate JS/CSS representations of base components.
- Be able to run a long-running process to listen for Figma changes and automatically create pull requests for code generation.
- Save `.figmaref` or keep some reference when outputting the component, so we can link Figma <-> output and prevent clobbering changes after initial export.
- What about handling component variants? Variable content? styled-components? JS options for output?
