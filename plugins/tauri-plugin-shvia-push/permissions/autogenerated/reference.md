## Default Permission

Permissões do push do ShvIA Mobile. Os comandos são chamados apenas pelo Rust
da casca (run_mobile_plugin) — nenhuma capability de webview os referencia.

#### This default permission set includes the following:

- `allow-watch-token`
- `allow-request-permission`

## Permission Table

<table>
<tr>
<th>Identifier</th>
<th>Description</th>
</tr>


<tr>
<td>

`shvia-push:allow-request-permission`

</td>
<td>

Enables the request_permission command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`shvia-push:deny-request-permission`

</td>
<td>

Denies the request_permission command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`shvia-push:allow-watch-token`

</td>
<td>

Enables the watch_token command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`shvia-push:deny-watch-token`

</td>
<td>

Denies the watch_token command without any pre-configured scope.

</td>
</tr>
</table>
