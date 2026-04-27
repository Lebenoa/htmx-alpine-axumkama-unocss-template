import { defineConfig, presetAttributify, presetWind4 } from 'unocss'

export default defineConfig({
    cli: {
        entry: {
            patterns: ['./templates/**/*.html', "./src/**/*.rs"],
            outFile: './public/styles.css',
            // rewrite: true,
        }
    },
    presets: [
        presetWind4(),
        presetAttributify({
            prefixedOnly: true
        })
    ],
    preflights: [
        {
            getCSS: () => `body { background-color: #151515; color: white; }`
        }
    ]
})
