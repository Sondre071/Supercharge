package blobstorage

import "encoding/xml"

type EnumerationResults struct {
	XMLName         xml.Name    `xml:"EnumerationResults"`
	ServiceEndpoint string      `xml:"ServiceEndpoint,attr"`
	Prefix          string      `xml:"Prefix"`
	Marker          string      `xml:"Marker"`
	MaxResults      int         `xml:"MaxResults"`
	Containers      []Container `xml:"Containers>Container"`
	NextMarker      string      `xml:"NextMarker"`
}

type Container struct {
	Name       string     `xml:"Name"`
	Version    string     `xml:"Version"`
	Deleted    bool       `xml:"Deleted"`
	Properties Properties `xml:"Properties"`
}

type Properties struct {
	LastModified           string `xml:"Last-Modified"`
	Etag                   string `xml:"Etag"`
	LeaseStatus            string `xml:"LeaseStatus"`
	LeaseState             string `xml:"LeaseState"`
	LeaseDuration          string `xml:"LeaseDuration"`
	PublicAccess           string `xml:"PublicAccess"`
	HasImmutabilityPolicy  bool   `xml:"HasImmutabilityPolicy"`
	HasLegalHold           bool   `xml:"HasLegalHold"`
	DeletedTime            string `xml:"DeletedTime"`
	RemainingRetentionDays int    `xml:"RemainingRetentionDays"`
}
