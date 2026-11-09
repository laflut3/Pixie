{{- define "pixie.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- define "pixie.fullname" -}}
{{- printf "%s-%s" .Release.Name (include "pixie.name" .) | trunc 63 | trimSuffix "-" -}}
{{- end -}}
