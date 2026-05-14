# Environment Variables Section Change

- The environment variable section should be merged with the API Section.
- The Header where the Api section is located should be updated to reflect the new section.
- User can use shortcuts related to each section to quickly navigate to the desired section.
- The header should highlight the name of the section which is currently active.
- Make the UI Polished and consistent with this new design.

## Environment Variables Section
- This section will contain and store all the environment variables for the collection.
- Based on which collection is selected, or which api/folder is selected, the environment variables will be loaded into the section.
- Users should be able to add, edit, and delete environment variables from this section.
- Environment variables should be stored in a persistent manner, so they are not lost when the user closes the application.
- Environment variable values should be masked or hidden from view with a shortcut key to toggle visibility.
- If Evnvironment variable is not implemented yet, create a plan to implement it, and then implement it.

## Smart Environment Variables Creation
- Add a feature to allow users to create smart environment variables that can be used across collections and requests.
- When a collection's requests all have the similar base URL, the user should be able to create a smart environment variable as baseURL that should replace the base URL in all requests in that collection to come from this smart environment variable.
- This baseURL env creation can be done by user while on the collection section by pressing a shortcut key. (:env create)
