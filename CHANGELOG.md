# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


### Added
- AI-powered changeset message generation with support for multiple providers (OpenAI and Gemini)
- Environment-based configuration for AI providers through `.env` file
- Automatic version calculation based on change type (MAJOR, MINOR, PATCH)

### Changed
- Improved changeset message formatting for better clarity and consistency
- Enhanced version management system with semantic versioning support

### Technical
- Added new modules for AI integration (`ai_calls`, `version_operations`)
- Implemented proper error handling for AI API calls
- Added message cleanup to ensure consistent formatting

