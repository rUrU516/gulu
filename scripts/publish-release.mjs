import { execFileSync } from 'node:child_process';
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const pkg = JSON.parse(readFileSync(join(root, 'package.json'), 'utf8'));
const config = JSON.parse(readFileSync(join(root, 'src-tauri/tauri.conf.json'), 'utf8'));
const cargo = readFileSync(join(root, 'src-tauri/Cargo.toml'), 'utf8');
const rustVersion = cargo.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
if (pkg.version !== config.version || rustVersion !== config.version) {
  throw new Error('package.json、Cargo.toml 和 tauri.conf.json 的版本号必须一致。');
}

const version = config.version;
const tag = `v${version}`;
const bundle = join(root, 'src-tauri/target/aarch64-apple-darwin/release/bundle');
const app = join(bundle, 'macos/gulu.app');
const dmg = join(bundle, `dmg/gulu_${version}_aarch64.dmg`);
const run = (command, args, env = {}) => execFileSync(command, args, {
  cwd: root, stdio: 'inherit', env: { ...process.env, ...env },
});
const read = (command, args) => execFileSync(command, args, {
  cwd: root, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'],
}).trim();

if (process.argv.includes('--dry-run')) {
  console.log(`准备发布 ${tag}：构建并校验 Apple 芯片 DMG，推送代码和标签，上传草稿后发布 Release。`);
  console.log(`安装包：${dmg}`);
  process.exit(0);
}

if (process.platform !== 'darwin') {
  throw new Error('当前 macOS 打包与签名脚本需要在 Mac 上运行。');
}
if (read('git', ['status', '--porcelain'])) {
  throw new Error('请先提交源码改动，再发布对应版本。');
}
read('git', ['remote', 'get-url', 'origin']);
run('gh', ['auth', 'status']);
const hasTag = read('git', ['tag', '--list', tag]);
if (hasTag && read('git', ['rev-parse', `${tag}^{commit}`]) !== read('git', ['rev-parse', 'HEAD'])) {
  throw new Error(`${tag} 已指向其他提交，请使用新的版本号。`);
}

run('pnpm', ['tauri', 'build', '--target', 'aarch64-apple-darwin', '--bundles', 'app,dmg'], {
  CI: 'true', APPLE_SIGNING_IDENTITY: '-',
});
run('codesign', ['--verify', '--deep', '--strict', app]);
run('hdiutil', ['verify', dmg]);
mkdirSync(join(root, 'releases'), { recursive: true });
copyFileSync(dmg, join(root, `releases/gulu-${version}-apple-silicon.dmg`));

if (!hasTag) run('git', ['tag', '-a', tag, '-m', `咕噜 ${version}`]);
const push = (ref) => run('git', [
  '-c', 'credential.helper=', '-c', 'credential.helper=!gh auth git-credential',
  'push', 'origin', ref,
]);
push('HEAD');
push(tag);

const notesDirectory = mkdtempSync(join(tmpdir(), 'gulu-release-'));
const notesPath = join(notesDirectory, 'notes.md');
try {
  writeFileSync(notesPath, `Apple 芯片 Mac 版咕噜。

- 每 5 分钟从菜单栏水滴弹出“记得喝水”，随后自动缩回。
- 菜单提供“试试提醒”“检查更新”和“退出”。

下载 .dmg，将 gulu 拖入“应用程序”后启动。已有版本请先选择“退出”，再替换并启动新版。

此包为本地签名，未经 Apple 公证。首次打开若提示开发者未验证，请参考 [Apple 官方说明](https://support.apple.com/zh-cn/102445)。
`);
  run('gh', ['release', 'create', tag, dmg, '--draft', '--verify-tag', '--title', `咕噜 ${tag}`, '--notes-file', notesPath]);
  run('gh', ['release', 'edit', tag, '--draft=false', '--latest']);
  run('gh', ['release', 'view', tag, '--json', 'url', '--jq', '.url']);
} finally {
  rmSync(notesDirectory, { recursive: true, force: true });
}
