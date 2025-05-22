import type { GlobalConfig } from 'semantic-release';

export default {
    branches: ['main'],
    plugins: [
        ['@semantic-release/commit-analyzer', {
            preset: 'conventionalcommits',
        }],
        ['@semantic-release/release-notes-generator', {
            preset: 'conventionalcommits',
            presetConfig: {
                types: [
                    { type: 'feat', section: ' :sparkles: 新機能' },
                    { type: 'fix', section: ' :wrench: 修正' },
                    {
                        type: 'perf',
                        section: ' :gem: パフォーマンス改善',
                    },
                    {
                        type: 'docs',
                        section: ' :memo: ドキュメント',
                        hidden: true,
                    },
                    {
                        type: 'style',
                        section: ' :barber: スタイル',
                        hidden: true,
                    },
                    {
                        type: 'chore',
                        section: 'その他',
                        hidden: true,
                    },
                    {
                        type: 'refactor',
                        section: ' :zap: リファクタリング',
                        hidden: true,
                    },
                    {
                        type: 'test',
                        section: ' :white_check_mark: テスト',
                        hidden: true,
                    },
                    {
                        type: 'ci',
                        section: ' :ci: Continuous Integration',
                        hidden: true,
                    },
                ],
            },
        }],
        '@semantic-release/changelog',
        ['@semantic-release/git', {
            assets: ['CHANGELOG.md', 'package.json', 'bun.lock', 'src-tauri/Cargo.toml', 'src-tauri/Cargo.lock'],
            message: 'chore(release): ${nextRelease.version} [skip ci]',
        }],
        ['@semantic-release/github', {
            assets: [
                'build/**/*.dmg',
                'build/**/*.msi',
            ],
        }],
    ],
    repositoryUrl: 'https://github.com/alinco8/yes-automatic',
    tagFormat: 'v${version}',
} satisfies GlobalConfig;
