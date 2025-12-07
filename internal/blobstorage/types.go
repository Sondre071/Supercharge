package blobstorage

import "encoding/xml"

type ContainerEnumerationResults struct {
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

// Root result for blob listing
type BlobEnumerationResults struct {
    XMLName         xml.Name `xml:"EnumerationResults"`
    ServiceEndpoint string   `xml:"ServiceEndpoint,attr"`
    ContainerName   string   `xml:"ContainerName,attr"`

    Prefix     string `xml:"Prefix"`
    Marker     string `xml:"Marker"`
    MaxResults int    `xml:"MaxResults"`

    Blobs      []Blob `xml:"Blobs>Blob"`
    NextMarker string `xml:"NextMarker"`
}

// Individual blob entry
type Blob struct {
    Name       string     `xml:"Name"`
    Snapshot   string     `xml:"Snapshot"`
    VersionId  string     `xml:"VersionId"`
    Properties BlobProps  `xml:"Properties"`
    Metadata   []MetaItem `xml:"Metadata"`
}

// Only properties we care about
type BlobProps struct {
    ContentLength int64  `xml:"Content-Length"`
    ContentMD5    string `xml:"Content-MD5"`
    LastModified  string `xml:"Last-Modified"`
    BlobType      string `xml:"BlobType"`
}

// Generic key/value metadata (Azure allows arbitrary keys)
type MetaItem struct {
    XMLName xml.Name
    Value   string `xml:",chardata"`
}