<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Post New Book Request</description>
   <name>Post New Book Request</name>
   <tag></tag>
   <elementGuidId>dd0a1ebd-8d9a-49dd-ae95-78da09dcc711</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ID\&quot;: \&quot;${book_id}\&quot;,\n  \&quot;Title\&quot;: \&quot;${book_title}\&quot;,\n  \&quot;Description\&quot;: \&quot;${book_description}\&quot;,\n  \&quot;PageCount\&quot;: 0,\n  \&quot;Excerpt\&quot;: \&quot;${book_excerpt}\&quot;,\n  \&quot;PublishDate\&quot;: \&quot;${books_published}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://fakerestapi.azurewebsites.net/api/Books</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>11</defaultValue>
      <description></description>
      <id>6f64baa1-13d1-4aca-b0d7-1abfcddd8f8c</id>
      <masked>false</masked>
      <name>book_id</name>
   </variables>
   <variables>
      <defaultValue>'Some Title'</defaultValue>
      <description></description>
      <id>2eda65d8-3c03-4e59-bc24-c232ebfcbc42</id>
      <masked>false</masked>
      <name>book_title</name>
   </variables>
   <variables>
      <defaultValue>'Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. Some Description. '</defaultValue>
      <description></description>
      <id>88306d4f-8ea1-4d57-810c-13a3f5c582cf</id>
      <masked>false</masked>
      <name>book_description</name>
   </variables>
   <variables>
      <defaultValue>'Book Excerpt text Sample. Book Excerpt text Sample. '</defaultValue>
      <description></description>
      <id>51cc4bab-4146-47aa-8b54-cbf68c7e6ff0</id>
      <masked>false</masked>
      <name>book_excerpt</name>
   </variables>
   <variables>
      <defaultValue>'2019-11-25T14:02:37.311Z'</defaultValue>
      <description></description>
      <id>20a19bfd-d4f7-43d9-b9bd-3f208a7da96f</id>
      <masked>false</masked>
      <name>books_published</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
