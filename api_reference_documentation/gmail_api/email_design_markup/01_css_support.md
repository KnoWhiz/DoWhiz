# CSS Support for Gmail

## Overview

Gmail supports styling emails using inline `<style>` blocks and standard CSS. Most CSS selectors, attributes, and media queries function properly, though some unsupported properties may be ignored.

## CSS Selectors

Gmail permits a limited subset of CSS selectors for applying styles: **class selectors, element selectors, and ID selectors**.

### Example

```html
<html>
  <head>
    <style>
      .colored {
        color: blue;
      }
      #body {
        font-size: 14px;
      }
    </style>
  </head>
  <body>
    <div id='body'>
      <p>Hi Pierce,</p>
      <p class='colored'>This text is blue.</p>
      <p>Jerry</p>
    </div>
  </body>
</html>
```

## CSS Media Queries

Standard CSS media queries allow responsive email styling based on device characteristics. Supported queries include screen width, orientation, and resolution parameters.

### Example

```html
<html>
  <head>
    <style>
      .colored {
        color: blue;
      }
      #body {
        font-size: 14px;
      }
      @media screen and (min-width: 500px) {
        .colored {
          color: red;
        }
      }
    </style>
  </head>
  <body>
    <div id='body'>
      <p>Hi Pierce,</p>
      <p class='colored'>
        This text is blue if the window width is below 500px and red otherwise.
      </p>
      <p>Jerry</p>
    </div>
  </body>
</html>
```

## Supported CSS Properties

The platform supports over 150 CSS properties, including background variants, border properties, color, display, font attributes, margins, padding, text formatting, and positioning options.

### Media Query Support

**Supported types:** `all`, `screen`

**Supported queries:** `min-width`, `max-width`, `min-device-width`, `max-device-width`, `orientation`, `min-resolution`, `max-resolution`

**Supported keywords:** `and`, `only`
