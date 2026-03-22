import { NapiCli } from '@napi-rs/cli'

new NapiCli().build({
  platform: true,
  dtsCache: false,
  release: false,
  constEnum: true,
  dts: './index.d.ts',
})
