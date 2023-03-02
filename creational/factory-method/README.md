# Dialog Rendering
This example illustrates how to organize a GUI framework into independent modules using <strong>dynamic dispatch</strong>:

1. The `gui` module defines interfaces for all the components. It has no external dependencies.
2. The `html_gui` module provides HTML implementation of the base GUI.
Depends on `gui`.
3. The windows_gui module provides Windows implementation of the base GUI.
Depends on `gui`.
The `app` is a client application that can use several implementations of the GUI framework, depending on the current environment or configuration. However, most of the app code doesn’t depend on specific types of GUI elements. All client code works with GUI elements through abstract interfaces defined by the `gui` module.