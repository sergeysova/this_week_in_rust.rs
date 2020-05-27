const core = require('@actions/core')
const { Octokit } = require('@octokit/action')

async function main() {
  const gistId = core.getInput('gist_id', { required: true })
  const fileName = core.getInput('file_name', { required: true })
  const octokit = new Octokit()

  const response = await octokit.gists.get({ gist_id: gistId })
  const { files } = response.data

  console.log(files[fileName])

  core.setOutput('last_id', files[fileName].content)
}


main().catch(error => {
  console.error(error)
  core.setFailed(error.message)
  process.exit(-1)
})
