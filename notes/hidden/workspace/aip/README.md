# `/workspace/aip`

Configured as the workspace `homeUrl`, but this is a server-side alias rather than a standalone page.

## Redirect behavior

```text
GET /workspace/aip
307 Location: aip/

GET /workspace/aip/
307 Location: /workspace/narrative/

GET /workspace/narrative/
200 OK
```

The first redirect adds the trailing slash. The second sends the browser to the active Foundry home application, `/workspace/narrative/`. This behavior occurs before client-side JavaScript runs and does not indicate a permission or application error.
