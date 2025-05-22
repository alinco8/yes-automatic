import pluginJs from '@eslint/js';
import pluginReact from 'eslint-plugin-react';
import pluginReactHooks from 'eslint-plugin-react-hooks';
import pluginReactRefresh from 'eslint-plugin-react-refresh';
import globals from 'globals';
import pluginTs, { config } from 'typescript-eslint';

export default config(
    pluginJs.configs.recommended,
    pluginTs.configs.strictTypeChecked,
    pluginReact.configs.flat.recommended,
    pluginReact.configs.flat['jsx-runtime'],
    pluginReactHooks.configs['recommended-latest'],
    pluginReactRefresh.configs.vite,
    {
        ignores: [
            '.wxt/**/*',
            '.output/**/*',
            'src/components/ui/**/*',
        ],
    },
    {
        files: ['**/*.{js,jsx,ts,tsx}'],
        languageOptions: {
            ecmaVersion: 2022,
            sourceType: 'module',
            globals: globals.browser,
            parserOptions: {
                projectService: true,
                tsconfigRootDir: import.meta.dirname,
            },
        },
        rules: {
            'no-duplicate-imports': 'error',
            '@typescript-eslint/no-dynamic-delete': 'off',
            '@typescript-eslint/unified-signatures': 'off',
            '@typescript-eslint/no-misused-promises': 'error',
            '@typescript-eslint/no-floating-promises': 'error',
            '@typescript-eslint/switch-exhaustiveness-check': ['error', {
                considerDefaultExhaustiveForUnions: true,
            }],
            '@typescript-eslint/no-deprecated': ['error', {
                allow: [
                    { from: 'file', name: 'chrome' },
                    { from: 'file', name: 'querySelectorAll' },
                ],
            }],
            '@typescript-eslint/restrict-template-expressions': ['error', {
                allowNumber: true,
            }],
            'react/self-closing-comp': 'error',
            'react/react-in-jsx-scope': 'off',
            'react/jsx-one-expression-per-line': 'off',
            'react/function-component-definition': 'off',
            'react/jsx-newline': 'off',
            'react/jsx-max-props-per-line': 'off',
            'react/jsx-sort-props': 'off',
            'react/jsx-no-bind': 'off',
            'react/jsx-no-literals': 'off',
            'react/jsx-max-depth': 'off',
            'react/jsx-props-no-spreading': 'off',
            'react/forbid-component-props': 'off',
            'react/jsx-indent': 'off',
            'react-refresh/only-export-components': [
                'error',
                {
                    allowExportNames: [
                        'meta',
                        'links',
                        'headers',
                        'loader',
                        'action',
                    ],
                },
            ],
        },
        settings: {
            react: { version: 'detect' },
        },
    },
);
