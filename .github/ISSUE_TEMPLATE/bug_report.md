name: Bug report
description: Create a report to help us improve
title: "[Bug]: "
labels: ["bug"]
body:
  - type: markdown
    attributes:
      value: |
        Thanks for taking the time to fill out this bug report!
  - type: textarea
    id: what-happened
    attributes:
      label: What happened?
      description: Also tell us, what did you expect to happen?
      placeholder: Tell us what you see!
    validations:
      required: true
  - type: textarea
    id: reproduction
    attributes:
      label: Steps to reproduce
      description: How do you trigger this bug? Please include commands, exact steps, and file structures if relevant.
      placeholder: |
        1. Run 'diskdoc'
        2. Navigate to '...'
        3. Press '...'
        4. See error
    validations:
      required: true
  - type: dropdown
    id: os
    attributes:
      label: Operating System
      options:
        - macOS
        - Linux
        - Other
    validations:
      required: true
  - type: textarea
    id: version
    attributes:
      label: Version
      description: What version of diskdoc are you running?
      placeholder: v0.1.0
  - type: textarea
    id: logs
    attributes:
      label: Relevant log output
      description: Please copy and paste any relevant log output. This will be automatically formatted into code, so no need for backticks.
      render: shell
